use bevy::log::error;
use bevy::prelude::{Reflect, Vec2};

use crate::bindings::InputBinding;
use crate::input_action::InputAction;
use crate::phantom::{DualAxis, IAWrp};
use crate::processed::bound_action::BoundAction;
use crate::processed::processor::Helper;
use crate::processed::stateful::axis_single::StatefulSingleAxisBinding;
use crate::processed::updating::InputSources;
use crate::resources::Ineffable;

#[derive(Debug, Reflect, Clone)]
pub(crate) struct StatefulDualAxisBinding {
    x: StatefulSingleAxisBinding,
    y: StatefulSingleAxisBinding,
    pub(crate) value: Vec2,
}

pub(crate) fn bound_action<I: InputAction>(
    ineffable: &Ineffable,
    input_action: IAWrp<I, DualAxis>,
) -> Option<&StatefulDualAxisBinding> {
    ineffable
        .groups.get(I::group_id())?.get(input_action.0.index())
        .and_then(|bound_action| {
            if let BoundAction::DualAxis(binding) = bound_action {
                Some(binding)
            } else {
                error!("Please use the ineff!() macro for a compile-time guarantee that you're using the correct InputKind.");
                None
            }
        })
}

impl StatefulDualAxisBinding {
    pub(crate) fn new(data: &[InputBinding], helper: &Helper<'_>) -> StatefulDualAxisBinding {
        let (vec_x, vec_y): (Vec<InputBinding>, Vec<InputBinding>) = data
            .iter()
            .filter_map(|binding| {
                if let InputBinding::DualAxis { x, y } = binding {
                    Some((
                        InputBinding::SingleAxis(x.clone()),
                        InputBinding::SingleAxis(y.clone()),
                    ))
                } else {
                    None
                }
            })
            .unzip();
        StatefulDualAxisBinding {
            x: StatefulSingleAxisBinding::new(&vec_x, helper),
            y: StatefulSingleAxisBinding::new(&vec_y, helper),
            value: Vec2::default(),
        }
    }
    pub(crate) fn update(&mut self, sources: &mut InputSources<'_>) {
        self.x.update(sources);
        self.y.update(sources);
        self.value = Vec2::new(self.x.value, self.y.value);
    }
}
