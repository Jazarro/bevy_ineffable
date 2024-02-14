use bevy::ecs::system::SystemParam;
use bevy::input::Input;
use bevy::prelude::{KeyCode, MouseButton, Res, ResMut, Time};

use crate::resources::ineffable_settings::IneffableSettings;
use crate::resources::Ineffable;

#[derive(SystemParam)]
pub(crate) struct InputSources<'w> {
    pub(crate) settings: ResMut<'w, IneffableSettings>,
    pub(crate) time: Res<'w, Time>,
    pub(crate) keys: Res<'w, Input<KeyCode>>,
    pub(crate) mouse_buttons: Res<'w, Input<MouseButton>>,
}

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
