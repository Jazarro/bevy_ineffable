use bevy::prelude::{GamepadAxis, Reflect};
use serde::{Deserialize, Serialize};

use crate::bindings::{BinaryInput, Threshold};

/// Input methods that indicate a direction and magnitude along a single axis.
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub enum AnalogInput {
    /// The amount of scrolling on the mouse's horizontal scroll wheel, since last tick.
    /// Note that most mice don't have this. You're probably looking for ScrollWheelY instead.
    ScrollWheelX,
    /// The amount of scrolling on the mouse's vertical scroll wheel, since last tick.
    ScrollWheelY,

    /// The amount of horizontal movement by the mouse since the last tick.
    MouseMotionX,
    /// The amount of vertical movement by the mouse since the last tick.
    MouseMotionY,

    /// Axis types specific to the GamePad.
    GamePad(GamepadAxis),
    /// The amount by which the bottom-left trigger is pushed in. Is a value between zero and one.
    /// This button is also present in the `GamepadButton`-enum, where it acts as a binary input that activates
    /// when the trigger is pushed 75% of the way in.
    GamePadLeftTrigger2,
    /// The amount by which the bottom-right trigger is pushed in. Is a value between zero and one.
    /// This button is also present in the `GamepadButton`-enum, where it acts as a binary input that activates
    /// when the trigger is pushed 75% of the way in.
    GamePadRightTrigger2,
}

impl AnalogInput {
    /// Converts an `AnalogInput` to a `BinaryInput` by applying a `Threshold`. Useful for the builder.
    pub fn at_threshold(self, threshold: Threshold) -> BinaryInput {
        BinaryInput::Axis(self, threshold)
    }
}
