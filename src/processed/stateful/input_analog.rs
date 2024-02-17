use bevy::prelude::{GamepadAxis, GamepadButton, GamepadButtonType, Reflect};

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

    pub(crate) fn update(&mut self, sources: &mut InputSources<'_>) {
        self.value_previous = self.value_current;

        // If the post-acceptance-delay is active, then do nothing. We should ignore all user input.
        if sources.settings.input_blocked_by_pad() {
            return;
        }
        self.value_current = Self::calc_value(&self.analog_input, sources);
    }
    pub(crate) fn calc_value(input: &AnalogInput, sources: &InputSources<'_>) -> f32 {
        match input {
            AnalogInput::ScrollWheelX => sources.from_events.mouse_scroll.x,
            AnalogInput::ScrollWheelY => sources.from_events.mouse_scroll.y,
            AnalogInput::MouseMotionX => sources.from_events.mouse_motion.x,
            AnalogInput::MouseMotionY => sources.from_events.mouse_motion.y,
            AnalogInput::GamePad(axis_type) => {
                // For now, we don't support local multiplayer. (Will change in the future)
                // We'll check if the button is active on *any* connected gamepad.
                sources
                    .gamepads
                    .iter()
                    .filter_map(|gamepad| {
                        sources
                            .axis_gamepad_axis
                            .get(GamepadAxis::new(gamepad, *axis_type))
                    })
                    .next()
                    .unwrap_or_default()
            }
            AnalogInput::GamePadLeftTrigger2 => {
                Self::gamepad_value(GamepadButtonType::LeftTrigger2, sources)
            }
            AnalogInput::GamePadRightTrigger2 => {
                Self::gamepad_value(GamepadButtonType::RightTrigger2, sources)
            }
        }
    }

    fn gamepad_value(btn: GamepadButtonType, sources: &InputSources<'_>) -> f32 {
        // For now, we don't support local multiplayer. (Will change in the future)
        // We'll check if the button is active on *any* connected gamepad.
        sources
            .gamepads
            .iter()
            .filter_map(|gamepad| {
                sources
                    .axis_gamepad_btn
                    .get(GamepadButton::new(gamepad, btn))
            })
            .next()
            .unwrap_or_default()
    }
}
