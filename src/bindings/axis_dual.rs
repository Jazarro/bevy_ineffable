use std::marker::PhantomData;

use bevy::log::error;
use bevy::prelude::Reflect;
use serde::{Deserialize, Serialize};

use crate::bindings::{InputBinding, SingleAxisBinding};
use crate::phantom::{DualAxis, IBWrp, SingleAxis};

#[derive(Debug, Default, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub struct DualAxisBinding;

impl DualAxisBinding {
    #[must_use]
    pub fn builder() -> DualAxisBindingBuilder {
        DualAxisBindingBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DualAxisBindingBuilder {
    x: Option<SingleAxisBinding>,
    y: Option<SingleAxisBinding>,
}

impl DualAxisBindingBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn set_x(mut self, input: IBWrp<SingleAxis>) -> Self {
        self.x = Some(Self::unwrap_axis(input));
        self
    }
    #[must_use]
    pub fn set_y(mut self, input: IBWrp<SingleAxis>) -> Self {
        self.y = Some(Self::unwrap_axis(input));
        self
    }
    #[must_use]
    pub fn build(self) -> IBWrp<DualAxis> {
        let binding = InputBinding::DualAxis {
            x: self.x.unwrap_or(SingleAxisBinding::Dummy),
            y: self.y.unwrap_or(SingleAxisBinding::Dummy),
        };
        IBWrp::<DualAxis>(binding, PhantomData)
    }

    #[must_use]
    fn unwrap_axis(input: IBWrp<SingleAxis>) -> SingleAxisBinding {
        if let InputBinding::Axis(axis) = input.0 {
            axis
        } else {
            // Because of the wrapper containing the PhantomData, this should normally never happen (because it
            // would be a compile error), but log this just in case someone misunderstood how to use the builder.
            error!("DualAxisBinding requires two single axis inputs: one for the x direction and one for y.\n\
            \tWe didn't get a single axis input, so this binding is not going to work.\n\
            \tTry to call this function like this:\n\
            \t\tDualAxisBinding::builder()\n\
            \t\t    .set_x(\n\
            \t\t        SingleAxisBinding::hold()\n\
            \t\t            .set_negative(KeyCode::A)\n\
            \t\t            .set_positive(KeyCode::D)\n\
            \t\t            .build(),\n\
            \t\t    )
            \t\t    .set_y(\n\
            \t\t        SingleAxisBinding::hold()\n\
            \t\t            .set_negative(KeyCode::S)\n\
            \t\t            .set_positive(KeyCode::W)\n\
            \t\t            .build(),\n\
            \t\t    )
            \t\t    .build()");
            SingleAxisBinding::Dummy
        }
    }
}
