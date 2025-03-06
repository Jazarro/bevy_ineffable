use bevy::prelude::{GamepadButton, Reflect};

use crate::bindings::AnalogInput;
use crate::processed::updating::InputSources;

#[derive(Debug, Reflect, Clone)]
pub(crate) struct StatefulAnalogInput {
    analog_input: AnalogInput,
    pub(crate) value_current: f32,
    pub(crate) value_previous: f32,
}

impl StatefulAnalogInput {
    pub fn new(input: &AnalogInput) -> Self {
        Self {
            analog_input: input.clone(),
            value_current: 0.,
            value_previous: 0.,
        }
    }

    pub(crate) fn just_activated(&self) -> bool {
        self.value_current.abs() > f32::EPSILON && self.value_previous.abs() < f32::EPSILON
    }

    pub(crate) fn update(&mut self, sources: &mut InputSources<'_, '_>) {
        self.value_previous = self.value_current;

        // If the post-acceptance-delay is active, then do nothing. We should ignore all user input.
        if sources.settings.input_blocked_by_pad() {
            return;
        }
        self.value_current = Self::calc_value(&self.analog_input, sources);
    }
    pub(crate) fn calc_value(input: &AnalogInput, sources: &InputSources<'_, '_>) -> f32 {
        match input {
            AnalogInput::ScrollWheelX => sources.mouse_scroll.delta.x,
            AnalogInput::ScrollWheelY => sources.mouse_scroll.delta.y,
            AnalogInput::MouseMotionX => sources.mouse_motion.delta.x,
            AnalogInput::MouseMotionY => sources.mouse_motion.delta.y,
            AnalogInput::GamePad(axis_type) => {
                // For now, we don't support local multiplayer. (Will change in the future)
                // We'll check if the button is active on *any* connected gamepad.
                sources
                    .gamepads
                    .iter()
                    .filter_map(|(_, gamepad)| gamepad.get(*axis_type))
                    .next()
                    .unwrap_or_default()
            }
            AnalogInput::GamePadLeftTrigger2 => {
                Self::gamepad_value(GamepadButton::LeftTrigger2, sources)
            }
            AnalogInput::GamePadRightTrigger2 => {
                Self::gamepad_value(GamepadButton::RightTrigger2, sources)
            }
        }
    }

    fn gamepad_value(btn: GamepadButton, sources: &InputSources<'_, '_>) -> f32 {
        // For now, we don't support local multiplayer. (Will change in the future)
        // We'll check if the button is active on *any* connected gamepad.
        sources
            .gamepads
            .iter()
            .filter_map(|(_, gamepad)| gamepad.get(btn))
            .next()
            .unwrap_or_default()
    }
}
