use std::ops::Not;

use bevy::log::error;
use bevy::prelude::Reflect;

use crate::bindings::{InputBinding, Inversion, PulseBinding, Sensitivity, SingleAxisBinding};
use crate::input_action::InputAction;
use crate::phantom::{IAWrp, SingleAxis};
use crate::processed::bound_action::BoundAction;
use crate::processed::processor::Helper;
use crate::processed::stateful::input_analog::StatefulAnalogInput;
use crate::processed::stateful::input_binary::StatefulBinaryInput;
use crate::processed::stateful::pulse::StatefulPulseBinding;
use crate::processed::stateful::{input_binary, pulse};
use crate::processed::updating::InputSources;
use crate::reporting::{ActionLocation, InputConfigProblem, InputConfigReport};
use crate::resources::meta_data::IneffableMetaItem;
use crate::resources::Ineffable;

#[derive(Debug, Default, Reflect, Clone)]
pub(crate) struct StatefulSingleAxisBinding {
    bindings: Vec<StatefulSingleAxisBindingVariant>,
    pub(crate) value: f32,
    toggled_direction: Direction1D,
}

#[derive(Debug, Reflect, Clone)]
pub(crate) enum StatefulSingleAxisBindingVariant {
    Dummy,
    /// Support for analog input devices is coming.
    Analog(StatefulAnalogInput, Inversion, Sensitivity),
    Held {
        negative: StatefulBinaryInput,
        positive: StatefulBinaryInput,
    },
    Toggle {
        negative: StatefulPulseBinding,
        positive: StatefulPulseBinding,
    },
}

pub(crate) fn bound_action<I: InputAction>(
    ineffable: &Ineffable,
    input_action: IAWrp<I, SingleAxis>,
) -> Option<&StatefulSingleAxisBinding> {
    ineffable
        .groups.get(I::group_id())?.get(input_action.0.index())
        .and_then(|bound_action| {
            if let BoundAction::SingleAxis(binding) = bound_action {
                Some(binding)
            } else {
                error!("Please use the ineff!() macro for a compile-time guarantee that you're using the correct InputKind.");
                None
            }
        })
}

pub(crate) fn collect<'a>(
    out: &mut Helper<'a>,
    meta: &'a IneffableMetaItem,
    binding: &SingleAxisBinding,
) {
    match binding {
        SingleAxisBinding::Dummy => {}
        SingleAxisBinding::Analog { .. } => {
            //todo
        }
        SingleAxisBinding::Hold(neg, pos) => {
            out.push(meta, neg.clone());
            out.push(meta, pos.clone());
        }
        SingleAxisBinding::Toggle(neg, pos) => {
            pulse::collect(out, meta, neg);
            pulse::collect(out, meta, pos);
        }
    }
}

pub(crate) fn check_for_problems(
    axis: &SingleAxisBinding,
    report: &mut InputConfigReport,
    loc: &ActionLocation,
) {
    match axis {
        SingleAxisBinding::Dummy => (),
        SingleAxisBinding::Analog { .. } => {
            //todo
        }
        SingleAxisBinding::Hold(neg, pos) => {
            if neg.is_empty() && pos.is_empty() {
                report.warning(InputConfigProblem::ConvolutedDummy {
                    loc: loc.clone(),
                    is_now: format!("{axis:?}"),
                });
            }
            input_binary::check_for_problems(neg, report, loc);
            input_binary::check_for_problems(pos, report, loc);
        }
        SingleAxisBinding::Toggle(neg, pos) => {
            if matches!(neg, PulseBinding::Dummy) && matches!(pos, PulseBinding::Dummy) {
                report.warning(InputConfigProblem::ConvolutedDummy {
                    loc: loc.clone(),
                    is_now: format!("{axis:?}"),
                });
            }
            pulse::check_for_problems(neg, report, loc);
            pulse::check_for_problems(pos, report, loc);
        }
    }
}

impl StatefulSingleAxisBinding {
    pub(crate) fn new(data: &[InputBinding], helper: &Helper<'_>) -> StatefulSingleAxisBinding {
        let stateful_bindings = data
            .iter()
            .filter_map(|binding| {
                if let InputBinding::SingleAxis(axis) = binding {
                    Some(axis)
                } else {
                    None
                }
            })
            .map(|axis| match axis {
                SingleAxisBinding::Dummy => StatefulSingleAxisBindingVariant::Dummy,
                SingleAxisBinding::Analog {
                    input,
                    inversion,
                    sensitivity,
                } => StatefulSingleAxisBindingVariant::Analog(
                    StatefulAnalogInput::new(input),
                    inversion.clone(),
                    sensitivity.clone(),
                ),
                SingleAxisBinding::Hold(negative, positive) => {
                    StatefulSingleAxisBindingVariant::Held {
                        negative: StatefulBinaryInput::new(negative, helper),
                        positive: StatefulBinaryInput::new(positive, helper),
                    }
                }
                SingleAxisBinding::Toggle(negative, positive) => {
                    StatefulSingleAxisBindingVariant::Toggle {
                        negative: StatefulPulseBinding::new_from_single(negative, helper),
                        positive: StatefulPulseBinding::new_from_single(positive, helper),
                    }
                }
            })
            .collect();
        StatefulSingleAxisBinding {
            bindings: stateful_bindings,
            value: 0.,
            toggled_direction: Direction1D::Neutral,
        }
    }
    pub(crate) fn update(&mut self, sources: &mut InputSources<'_, '_>) {
        let (min, max, toggle_neg, toggle_pos, newly_held) = self.bindings.iter_mut().fold(
            (0., 0., false, false, false),
            |(min, max, toggle_neg, toggle_pos, newly_held), binding| match binding {
                StatefulSingleAxisBindingVariant::Dummy => {
                    (min, max, toggle_neg, toggle_pos, newly_held)
                }
                StatefulSingleAxisBindingVariant::Analog(input, inversion, sensitivity) => {
                    input.update(sources);
                    let value =
                        input.value_current * inversion.multiplier() * sensitivity.multiplier();
                    (
                        value.min(min),
                        value.max(max),
                        toggle_neg,
                        toggle_pos,
                        newly_held || input.just_activated(),
                    )
                }
                StatefulSingleAxisBindingVariant::Held { negative, positive } => {
                    negative.update(sources);
                    positive.update(sources);
                    (
                        if negative.is_active() {
                            min.min(-1.)
                        } else {
                            min
                        },
                        if positive.is_active() {
                            max.max(1.)
                        } else {
                            max
                        },
                        toggle_neg,
                        toggle_pos,
                        newly_held || negative.just_pressed() || positive.just_pressed(),
                    )
                }
                StatefulSingleAxisBindingVariant::Toggle { negative, positive } => {
                    negative.update(sources);
                    positive.update(sources);
                    (
                        min,
                        max,
                        toggle_neg || negative.just_pulsed,
                        toggle_pos || positive.just_pulsed,
                        newly_held,
                    )
                }
            },
        );
        let toggle = Direction1D::from_input(toggle_neg, toggle_pos);
        if !matches!(toggle, Direction1D::Neutral) {
            self.toggled_direction = self.toggled_direction.toggle(toggle);
        } else if newly_held {
            self.toggled_direction = Direction1D::Neutral;
        }
        self.value = if !matches!(self.toggled_direction, Direction1D::Neutral) {
            self.toggled_direction.signum()
        } else if min < 0. && max > 0. {
            0.
        } else if min < 0. {
            min
        } else {
            max
        };
    }
}

// =====================================================================================================================
// ===== Direction helper enum:
// =====================================================================================================================

#[derive(Debug, Default, Reflect, Clone, Copy, PartialEq, Eq)]
enum Direction1D {
    Negative,
    Positive,
    #[default]
    Neutral,
}

impl Direction1D {
    #[must_use]
    fn from_input(negative: bool, positive: bool) -> Self {
        if negative ^ positive {
            if negative {
                Direction1D::Negative
            } else {
                Direction1D::Positive
            }
        } else {
            Direction1D::Neutral
        }
    }
    #[must_use]
    fn toggle(self, toggle: Direction1D) -> Self {
        if self == toggle {
            Direction1D::Neutral
        } else if toggle == Direction1D::Neutral {
            self
        } else if self == Direction1D::Neutral {
            toggle
        } else {
            !self
        }
    }
    #[must_use]
    fn signum(&self) -> f32 {
        match self {
            Direction1D::Positive => 1.,
            Direction1D::Negative => -1.,
            Direction1D::Neutral => 0.,
        }
    }
}

impl Not for Direction1D {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction1D::Negative => Direction1D::Positive,
            Direction1D::Neutral => Direction1D::Neutral,
            Direction1D::Positive => Direction1D::Negative,
        }
    }
}
