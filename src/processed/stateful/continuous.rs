use std::time::Duration;

use bevy::log::error;
use bevy::prelude::Reflect;
use bevy::time::Stopwatch;
use bevy::utils::default;

use crate::bindings::{ContinuousBinding, InputBinding, PulseBinding};
use crate::input_action::InputAction;
use crate::phantom::{Continuous, IAWrp};
use crate::processed::bound_action::BoundAction;
use crate::processed::processor::Helper;
use crate::processed::stateful::input_binary::StatefulBinaryInput;
use crate::processed::stateful::pulse::StatefulPulseBinding;
use crate::processed::stateful::{input_binary, pulse};
use crate::processed::updating::InputSources;
use crate::reporting::{ActionLocation, InputConfigProblem, InputConfigReport};
use crate::resources::meta_data::IneffableMetaItem;
use crate::resources::Ineffable;

// /// Used for the preset charged variant. This is the minimum charge duration that can be set.
// /// This is based on roughly how quickly a human could tap and let go of a key in the real world.
// /// Setting this as a floor prevents strange glitches caused by mistaken assumptions about the charge length.
// pub const MINIMUM_CHARGE_DURATION_MILLIS: u64 = 50;

#[derive(Debug, Reflect, Default, Clone)]
pub(crate) struct StatefulContinuousBinding {
    bindings: Vec<StatefulContinuousBindingVariant>,
    toggled_on: bool,
    pub(crate) active: bool,
    pub(crate) active_previous_tick: bool,
    time_active: Stopwatch,
}

#[derive(Debug, Reflect, Clone)]
pub(crate) enum StatefulContinuousBindingVariant {
    Dummy,
    Held(StatefulBinaryInput),
    Toggle(StatefulPulseBinding),
}

pub(crate) fn bound_action<I: InputAction>(
    ineffable: &Ineffable,
    input_action: IAWrp<I, Continuous>,
) -> Option<&StatefulContinuousBinding> {
    ineffable
        .groups.get(I::group_id())?.get(input_action.0.index())
        .and_then(|bound_action| {
            if let BoundAction::Continuous(binding) = bound_action {
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
    binding: &ContinuousBinding,
) {
    match binding {
        ContinuousBinding::Dummy => {}
        ContinuousBinding::Hold(input) => {
            out.push(meta, input.clone());
        }
        ContinuousBinding::Toggle(pulse) => {
            pulse::collect(out, meta, pulse);
        }
    }
}

pub(crate) fn check_for_problems(
    continuous: &ContinuousBinding,
    report: &mut InputConfigReport,
    loc: &ActionLocation,
) {
    match continuous {
        ContinuousBinding::Dummy => {}
        ContinuousBinding::Hold(input) => {
            if input.is_empty() {
                report.warning(InputConfigProblem::ConvolutedDummy {
                    loc: loc.clone(),
                    is_now: format!("{continuous:?}"),
                });
            }
            input_binary::check_for_problems(input, report, loc);
        }
        ContinuousBinding::Toggle(pulse) => {
            if matches!(pulse, PulseBinding::Dummy) {
                report.warning(InputConfigProblem::ConvolutedDummy {
                    loc: loc.clone(),
                    is_now: format!("{continuous:?}"),
                });
            }
            pulse::check_for_problems(pulse, report, loc);
        }
    }
}

impl StatefulContinuousBinding {
    pub(crate) fn new(data: &[InputBinding], helper: &Helper<'_>) -> StatefulContinuousBinding {
        let stateful_bindings = data
            .iter()
            .filter_map(|binding| {
                if let InputBinding::Continuous(continuous) = binding {
                    Some(continuous)
                } else {
                    None
                }
            })
            .map(|continuous| match continuous {
                ContinuousBinding::Dummy => StatefulContinuousBindingVariant::Dummy,
                ContinuousBinding::Hold(binary_input) => StatefulContinuousBindingVariant::Held(
                    StatefulBinaryInput::new(binary_input, helper),
                ),
                ContinuousBinding::Toggle(pulse) => StatefulContinuousBindingVariant::Toggle(
                    StatefulPulseBinding::new_from_single(pulse, helper),
                ),
            })
            .collect();
        StatefulContinuousBinding {
            bindings: stateful_bindings,
            ..default()
        }
    }
    pub(crate) fn update(&mut self, sources: &mut InputSources<'_>) {
        self.active_previous_tick = self.active;
        let (held, just_pressed, toggle) = self.bindings.iter_mut().fold(
            (false, false, false),
            |(held, just_pressed, toggle), binding| match binding {
                StatefulContinuousBindingVariant::Dummy => (held, just_pressed, toggle),
                StatefulContinuousBindingVariant::Held(input) => {
                    input.update(sources);
                    (
                        held || input.is_active(),
                        just_pressed || input.just_pressed(),
                        toggle,
                    )
                }
                StatefulContinuousBindingVariant::Toggle(toggle_control) => {
                    toggle_control.update(sources);
                    (held, just_pressed, toggle || toggle_control.just_pulsed)
                }
            },
        );
        if toggle {
            self.toggled_on = !self.toggled_on;
        } else if just_pressed {
            // If you start holding down a binding it should break existing toggles.
            // Otherwise players might wonder why letting go of the control doesn't stop the action.
            self.toggled_on = false;
        }
        self.active = held || self.toggled_on;
        if self.active {
            self.time_active.tick(sources.time.delta());
        } else {
            self.time_active.reset();
        }
    }

    pub fn charging_duration(&self) -> Option<Duration> {
        if self.time_active.elapsed().is_zero() {
            None
        } else {
            Some(self.time_active.elapsed())
        }
    }
}
