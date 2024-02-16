use bevy::math::Vec2;
use bevy::prelude::Resource;

/// Used by Ineffable to group information about miscellaneous input sources.
/// This is info that comes in through events.
#[derive(Debug, Default, Resource)]
pub struct IneffableEventSources {
    pub mouse_motion: Vec2,
    pub mouse_scroll: Vec2,
}

impl IneffableEventSources {
    pub fn clear(&mut self) {
        self.mouse_motion = Vec2::default();
        self.mouse_scroll = Vec2::default();
    }
}
