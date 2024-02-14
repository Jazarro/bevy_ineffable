use bevy::prelude::Reflect;
use serde::{Deserialize, Serialize};

use crate::bindings::{ContinuousBinding, PulseBinding, SingleAxisBinding};
use crate::input_action::InputKind;

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub enum InputBinding {
    Axis(SingleAxisBinding),
    DualAxis {
        x: SingleAxisBinding,
        y: SingleAxisBinding,
    },
    Continuous(ContinuousBinding),
    Pulse(PulseBinding),
}

impl InputBinding {
    #[must_use]
    pub fn kind(&self) -> InputKind {
        match self {
            InputBinding::Axis(_) => InputKind::SingleAxis,
            InputBinding::DualAxis { .. } => InputKind::DualAxis,
            InputBinding::Continuous(_) => InputKind::Continuous,
            InputBinding::Pulse(_) => InputKind::Pulse,
        }
    }
}

// TODO: KeyGroup
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub enum KeyGroup {
    Enter,
    Control,
    Shift,
    Alt,
}

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub enum AnalogAxisInput {
    ScrollWheelX,
    ScrollWheelY,
    MousePositionX,
    MousePositionY,
    GamePadStickLeftX,
    GamePadStickLeftY,
    GamePadStickRightX,
    GamePadStickRightY,
}

#[derive(Debug, Default, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub enum AxisInversion {
    #[default]
    NotInverted,
    Inverted,
}

// TODO:....
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub struct AxisOptions {
    // pub positive_low: f32,
    // pub negative_low: f32,
    // pub sensitivity: f32,
}

// TODO:...
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub struct AxisDeadZone {}
