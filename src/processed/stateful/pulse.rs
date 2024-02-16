use bevy::log::error;
use bevy::prelude::Reflect;
use bevy::time::Stopwatch;

use crate::bindings::{InputBinding, PulseBinding};
use crate::input_action::InputAction;
use crate::phantom::{IAWrp, Pulse};
use crate::processed::bound_action::BoundAction;
use crate::processed::processor::Helper;
use crate::processed::stateful::input_binary;
use crate::processed::stateful::input_binary::StatefulBinaryInput;
use crate::processed::updating::InputSources;
use crate::reporting::{ActionLocation, InputConfigProblem, InputConfigReport};
use crate::resources::meta_data::IneffableMetaItem;
use crate::resources::Ineffable;

#[derive(Debug, Reflect, Clone)]
pub(crate) struct StatefulPulseBinding {
    bindings: Vec<StatefulPulseBindingVariant>,
    pub(crate) just_pulsed: bool,
}

#[derive(Debug, Reflect, Clone)]
pub(crate) enum StatefulPulseBindingVariant {
    Dummy,
    JustPressed(StatefulBinaryInput),
    JustReleased(StatefulBinaryInput),
    DoubleClick {
        input: StatefulBinaryInput,
        timer: Stopwatch,
        index: usize,
    },
    Sequence {
        inputs: Vec<StatefulBinaryInput>,
        timeout: u128,
        timer: Stopwatch,
        index: usize,
    },
}

pub(crate) fn bound_action<I: InputAction>(
    ineffable: &Ineffable,
    input_action: IAWrp<I, Pulse>,
) -> Option<&StatefulPulseBinding> {
    ineffable
        .groups.get(I::group_id())?.get(input_action.0.index())
        .and_then(|bound_action| {
            if let BoundAction::Pulse(binding) = bound_action {
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
    binding: &PulseBinding,
) {
    match binding {
        PulseBinding::Dummy => (),
        PulseBinding::JustPressed(input)
        | PulseBinding::JustReleased(input)
        | PulseBinding::DoubleClick(input) => {
            out.push(meta, input.clone());
        }
        PulseBinding::Sequence(_, inputs) => {
            for input in inputs {
                out.push(meta, input.clone());
            }
        }
    }
}

pub(crate) fn check_for_problems(
    pulse: &PulseBinding,
    report: &mut InputConfigReport,
    loc: &ActionLocation,
) {
    match pulse {
        PulseBinding::Dummy => (),
        PulseBinding::JustPressed(input)
        | PulseBinding::JustReleased(input)
        | PulseBinding::DoubleClick(input) => {
            if input.is_empty() {
                report.warning(InputConfigProblem::ConvolutedDummy {
                    loc: loc.clone(),
                    is_now: format!("{pulse:?}"),
                });
            }
            input_binary::check_for_problems(input, report, loc);
        }
        PulseBinding::Sequence(millis, inputs) => {
            if *millis <= 25 {
                report.error(InputConfigProblem::SequenceUnrealisticTiming {
                    loc: loc.clone(),
                    actual_millis: *millis as usize,
                });
            }
            if inputs.is_empty() {
                report.warning(InputConfigProblem::SequenceEmpty { loc: loc.clone() });
            }
            if inputs.len() == 1 {
                report.warning(InputConfigProblem::SequenceOnlyContainsOneElement {
                    loc: loc.clone(),
                });
            }
            if inputs.iter().any(Vec::is_empty) {
                report.error(InputConfigProblem::ConvolutedDummy {
                    loc: loc.clone(),
                    is_now: format!("{pulse:?}"),
                });
            }
            for child in inputs {
                input_binary::check_for_problems(child, report, loc);
            }
        }
    }
}

impl StatefulPulseBinding {
    pub(crate) fn new_from_vec(data: &[InputBinding], helper: &Helper<'_>) -> StatefulPulseBinding {
        let stateful_bindings = data
            .iter()
            .filter_map(|binding| {
                if let InputBinding::Pulse(pulse) = binding {
                    Some(pulse)
                } else {
                    None
                }
            })
            .map(|value| Self::process(value, helper))
            .collect();
        StatefulPulseBinding {
            bindings: stateful_bindings,
            just_pulsed: false,
        }
    }
    pub(crate) fn new_from_single(
        value: &PulseBinding,
        helper: &Helper<'_>,
    ) -> StatefulPulseBinding {
        Self {
            bindings: vec![Self::process(value, helper)],
            just_pulsed: false,
        }
    }
    fn process(binding: &PulseBinding, helper: &Helper<'_>) -> StatefulPulseBindingVariant {
        match binding {
            PulseBinding::Dummy => StatefulPulseBindingVariant::Dummy,
            PulseBinding::JustPressed(input) => {
                StatefulPulseBindingVariant::JustPressed(StatefulBinaryInput::new(input, helper))
            }
            PulseBinding::JustReleased(input) => {
                StatefulPulseBindingVariant::JustReleased(StatefulBinaryInput::new(input, helper))
            }
            PulseBinding::DoubleClick(input) => StatefulPulseBindingVariant::DoubleClick {
                input: StatefulBinaryInput::new(input, helper),
                timer: Stopwatch::default(),
                index: 0,
            },
            PulseBinding::Sequence(timeout, inputs) => StatefulPulseBindingVariant::Sequence {
                inputs: inputs
                    .iter()
                    .map(|input| StatefulBinaryInput::new(input, helper))
                    .collect(),
                timeout: u128::from(*timeout),
                timer: Stopwatch::default(),
                index: 0,
            },
        }
    }

    pub(crate) fn update(&mut self, sources: &mut InputSources<'_>) {
        self.just_pulsed =
            self.bindings
                .iter_mut()
                .fold(false, |activated, binding| match binding {
                    StatefulPulseBindingVariant::Dummy => false,
                    StatefulPulseBindingVariant::JustPressed(input) => {
                        input.update(sources);
                        activated || input.just_pressed()
                    }
                    StatefulPulseBindingVariant::JustReleased(input) => {
                        input.update(sources);
                        activated || input.just_released()
                    }
                    StatefulPulseBindingVariant::DoubleClick {
                        input,
                        timer,
                        index,
                    } => {
                        input.update(sources);
                        let advance = input.just_pressed();
                        let timed_out = timer.elapsed() > sources.settings.double_click_timing;
                        if timed_out {
                            timer.reset();
                            *index = 0;
                        } else if advance && 2 <= *index + 1 {
                            timer.reset();
                            *index = 0;
                            return true;
                        } else if advance {
                            timer.reset();
                            *index += 1;
                        } else if *index > 0 {
                            timer.tick(sources.time.delta());
                        }
                        activated
                    }
                    StatefulPulseBindingVariant::Sequence {
                        inputs,
                        timeout,
                        timer,
                        index,
                    } => {
                        inputs.iter_mut().for_each(|input| input.update(sources));
                        let advance = inputs
                            .get(*index)
                            .is_some_and(StatefulBinaryInput::just_pressed);
                        let timed_out = timer.elapsed().as_millis() > *timeout;
                        if timed_out {
                            timer.reset();
                            *index = 0;
                        } else if advance && inputs.len() <= *index + 1 {
                            timer.reset();
                            *index = 0;
                            return true;
                        } else if advance {
                            timer.reset();
                            *index += 1;
                        } else if *index > 0 {
                            timer.tick(sources.time.delta());
                        }
                        activated
                    }
                });
    }
}
