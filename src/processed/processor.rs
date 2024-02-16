use crate::bindings::{Chord, ContinuousBinding, InputBinding, PulseBinding, SingleAxisBinding};
use crate::config::InputConfig;
use crate::input_action::InputKind;
use crate::processed::stateful::{axis_single, continuous, pulse};
use crate::reporting::{ActionLocation, InputConfigProblem, InputConfigReport};
use crate::resources::meta_data::{IneffableMetaData, IneffableMetaItem};

#[derive(Debug, Default)]
pub(crate) struct Helper<'a> {
    pub(crate) inputs: Vec<(&'a IneffableMetaItem, Chord)>,
}

impl<'a> Helper<'a> {
    pub(crate) fn push(&mut self, meta: &'a IneffableMetaItem, input: Chord) {
        self.inputs.push((meta, input));
    }
}

pub(crate) fn collect_inputs<'a>(
    meta_data: &'a IneffableMetaData,
    config: &'a InputConfig,
) -> Helper<'a> {
    let mut out = Helper::default();
    config
        .bindings
        .iter()
        .filter(|(group_id, _)| meta_data.group_exists(group_id))
        .flat_map(|(group_id, group_data)| {
            group_data.into_iter().filter_map(|(action_id, bindings)| {
                meta_data
                    .action(group_id, action_id)
                    .map(|meta| (meta, bindings))
            })
        })
        .flat_map(|(meta, bindings)| {
            bindings
                .iter()
                .map(move |input_binding| (meta, input_binding))
        })
        .for_each(|(meta, binding)| match meta.kind {
            InputKind::SingleAxis => {
                if let InputBinding::SingleAxis(axis) = binding {
                    axis_single::collect(&mut out, meta, axis);
                };
            }
            InputKind::DualAxis => {
                if let InputBinding::DualAxis { x, y } = binding {
                    axis_single::collect(&mut out, meta, x);
                    axis_single::collect(&mut out, meta, y);
                };
            }
            InputKind::Continuous => {
                if let InputBinding::Continuous(continuous) = binding {
                    continuous::collect(&mut out, meta, continuous);
                };
            }
            InputKind::Pulse => {
                if let InputBinding::Pulse(pulse) = binding {
                    pulse::collect(&mut out, meta, pulse);
                };
            }
        });
    out
}

#[must_use]
pub(crate) fn validate(meta_data: &IneffableMetaData, config: &InputConfig) -> InputConfigReport {
    let mut report = InputConfigReport::default();
    for (group_id, groups) in &config.bindings {
        if !meta_data.group_exists(group_id) {
            report.error(InputConfigProblem::UnknownGroup {
                group_id: group_id.to_string(),
                options: meta_data.group_ids(),
            });
            continue;
        }
        for (action_id, bindings) in groups {
            let Some(registered_item) = meta_data.action(group_id, action_id) else {
                report.error(InputConfigProblem::UnknownAction {
                    group_id: group_id.to_string(),
                    action_id: action_id.to_string(),
                    options: meta_data.action_ids(group_id),
                });
                continue;
            };
            for (index, binding) in bindings.iter().enumerate() {
                let loc = ActionLocation {
                    group_id: group_id.to_string(),
                    action_id: action_id.to_string(),
                    index,
                };
                let kind_from_config = binding.kind();
                if kind_from_config != registered_item.kind {
                    report.error(InputConfigProblem::ActionWrongKind {
                        loc: ActionLocation {
                            group_id: group_id.to_string(),
                            action_id: action_id.to_string(),
                            index,
                        },
                        wrong_kind: kind_from_config,
                        right_kind: registered_item.kind,
                    });
                }

                match binding {
                    InputBinding::SingleAxis(axis) => {
                        if matches!(axis, SingleAxisBinding::Dummy) {
                            report.warning(InputConfigProblem::RootBindingIsDummy {
                                loc: loc.clone(),
                            });
                        }
                        axis_single::check_for_problems(axis, &mut report, &loc);
                    }
                    InputBinding::DualAxis { x, y } => {
                        if matches!(x, SingleAxisBinding::Dummy)
                            && matches!(y, SingleAxisBinding::Dummy)
                        {
                            report.warning(InputConfigProblem::RootBindingIsDummy {
                                loc: loc.clone(),
                            });
                        }
                        axis_single::check_for_problems(x, &mut report, &loc);
                        axis_single::check_for_problems(y, &mut report, &loc);
                    }
                    InputBinding::Continuous(continuous) => {
                        if matches!(continuous, ContinuousBinding::Dummy) {
                            report.warning(InputConfigProblem::RootBindingIsDummy {
                                loc: loc.clone(),
                            });
                        }
                        continuous::check_for_problems(continuous, &mut report, &loc);
                    }
                    InputBinding::Pulse(pulse) => {
                        if matches!(pulse, PulseBinding::Dummy) {
                            report.warning(InputConfigProblem::RootBindingIsDummy {
                                loc: loc.clone(),
                            });
                        }
                        pulse::check_for_problems(pulse, &mut report, &loc);
                    }
                }
            }
        }
    }
    // TODO: Warn conflicts.
    report
}
