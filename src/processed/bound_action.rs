use bevy::prelude::Reflect;

use crate::bindings::InputBinding;
use crate::input_action::InputKind;
use crate::processed::processor::Helper;
use crate::processed::stateful::axis_dual::StatefulDualAxisBinding;
use crate::processed::stateful::axis_single::StatefulSingleAxisBinding;
use crate::processed::stateful::continuous::StatefulContinuousBinding;
use crate::processed::stateful::pulse::StatefulPulseBinding;
use crate::processed::updating::InputSources;
use crate::resources::meta_data::IneffableMetaItem;

#[derive(Debug, Reflect, Clone)]
pub(crate) enum BoundAction {
    SingleAxis(StatefulSingleAxisBinding),
    DualAxis(StatefulDualAxisBinding),
    Continuous(StatefulContinuousBinding),
    Pulse(StatefulPulseBinding),
}

impl BoundAction {
    pub(crate) fn new(
        meta: &IneffableMetaItem,
        data: &[InputBinding],
        helper: &Helper<'_>,
    ) -> BoundAction {
        match meta.kind {
            InputKind::SingleAxis => {
                BoundAction::SingleAxis(StatefulSingleAxisBinding::new(data, helper))
            }
            InputKind::DualAxis => {
                BoundAction::DualAxis(StatefulDualAxisBinding::new(data, helper))
            }
            InputKind::Continuous => {
                BoundAction::Continuous(StatefulContinuousBinding::new(data, helper))
            }
            InputKind::Pulse => {
                BoundAction::Pulse(StatefulPulseBinding::new_from_vec(data, helper))
            }
        }
    }

    pub(crate) fn update(&mut self, sources: &mut InputSources<'_>) {
        match self {
            BoundAction::SingleAxis(binding) => binding.update(sources),
            BoundAction::DualAxis(binding) => binding.update(sources),
            BoundAction::Continuous(binding) => binding.update(sources),
            BoundAction::Pulse(binding) => binding.update(sources),
        };
    }
}
