use bevy::math::Vec2;
use bevy::prelude::Resource;

/// Used by Ineffable to group information about miscellaneous input sources.
/// This is info that comes in through events.
#[derive(Debug, Default, Resource)]
pub struct IneffableEventSources {
    /// The distance that the cursor has moved since the last tick.
    /// According to `MouseMotion` documentation: "This represents raw, unfiltered physical motion."
    /// So probably distance in pixels??
    pub mouse_motion: Vec2,
    /// The distance in lines scrolled.
    /// When scrolling the mouse wheel, this tends to give values of one or two.
    pub mouse_scroll: Vec2,
}

impl IneffableEventSources {
    /// Reset all data to zero.
    pub(crate) fn clear(&mut self) {
        self.mouse_motion = Vec2::default();
        self.mouse_scroll = Vec2::default();
    }
}
