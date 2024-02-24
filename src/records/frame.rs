use bevy::math::Vec2;
use bevy::prelude::Reflect;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

use crate::config::DurationInMillis;
use crate::prelude::PulseBinding;

#[derive(Debug, Default, Serialize, Deserialize, Reflect, Clone, PartialEq)]
pub struct InputMacro {
    /// Optional name. Could be used by the player to identify the macro, if the game dev chooses
    /// to display macros in the GUI.
    #[serde(default)]
    pub name: String,
    pub trigger: PulseBinding,
    pub recording: HashMap<DurationInMillis, Vec<InputFrame>>,
}

/// First argument is the action id: a String of format: "ExampleInput::Example".
/// Second argument is the change that is enacted.
pub type InputFrame = (String, ActionDelta);

/// A change to an `InputAction`.
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
pub enum ActionDelta {
    /// The new Vec2 direction that is output by the DualAxis action.
    /// The output will retain this value until it is altered again.
    DualAxis(Vec2),
    /// The new f32 direction that is output by the SingleAxis action.
    /// The output will retain this value until it is altered again.
    SingleAxis(f32),
    /// Toggles the continuous action on or off.
    /// This is idempotent, toggling an already on state does nothing.
    Continuous(ContinuousStateChange),
    /// The pulse action will pulse.
    Pulse,
}

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
pub enum ContinuousStateChange {
    Start,
    End,
}
