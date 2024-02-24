use bevy::ecs::system::SystemParam;
use bevy::log::{error, info, warn};
use bevy::prelude::*;

use crate::resources::ineffable_settings::IneffableSettings;
use crate::resources::sources::IneffableEventSources;
use crate::resources::Ineffable;

#[derive(SystemParam)]
pub(crate) struct InputSources<'w> {
    pub(crate) settings: ResMut<'w, IneffableSettings>,
    pub(crate) time: Res<'w, Time>,
    pub(crate) from_events: Res<'w, IneffableEventSources>,
    pub(crate) gamepads: Res<'w, Gamepads>,
    pub(crate) input_keycodes: Res<'w, ButtonInput<KeyCode>>,
    pub(crate) input_mouse_btn: Res<'w, ButtonInput<MouseButton>>,
    pub(crate) input_gamepad_btn: Res<'w, ButtonInput<GamepadButton>>,
    pub(crate) axis_gamepad_btn: Res<'w, Axis<GamepadButton>>,
    pub(crate) axis_gamepad_axis: Res<'w, Axis<GamepadAxis>>,
}

/// Does the input from live input sources.
#[allow(clippy::needless_pass_by_value)] // That's just a bevy thing.
pub(crate) fn update_input(mut bindings: ResMut<'_, Ineffable>, mut sources: InputSources<'_>) {
    bindings
        .groups
        .iter_mut()
        .flat_map(|(_, group)| group.iter_mut())
        .for_each(|bound_action| {
            bound_action.update(&mut sources);
        });
    if let Some(pad) = &mut sources.settings.post_acceptance_delay {
        pad.tick(sources.time);
    }
}

/// Does the input from playbacks (macros, etc).
pub(crate) fn update_playback(mut bindings: ResMut<'_, Ineffable>) {
    //TODO:
    // Update the stateful pulse trigger for macros. (Maybe its own system??)
    // Then if it pulses, start the macro playback.
    // For a running playback, if it is active in this context, update the timer and execute any frames that are up.
    // When executing frames; go into the processed stuff and adjust a special playback binding.
}

// TODO: Remove.
pub(crate) fn _peek_at_input(sources: InputSources<'_>) {
    for btn in sources.input_mouse_btn.get_just_pressed() {
        warn!("JustPressed: {:?}", btn);
    }
    for btn in sources.input_keycodes.get_just_pressed() {
        info!("JustPressed: {:?}", btn);
    }
    for btn in sources.input_keycodes.get_pressed() {
        error!("Pressed: {:?}", btn);
    }
}
