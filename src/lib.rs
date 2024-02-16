#![forbid(unsafe_code)]
#![forbid(unstable_features)]
// #![forbid(missing_docs)] // TODO: enable.
#![deny(
rust_2018_compatibility,
rust_2018_idioms,
unused,
nonstandard_style,
future_incompatible,
missing_debug_implementations,
// clippy::all,
// clippy::pedantic
)]
#![doc = include_str!("../README.md")]

pub mod bindings;
pub mod commands;
pub mod config;
pub mod input_action;
pub mod phantom;
pub mod plugin;
pub mod processed;
pub mod register;
pub mod reporting;
pub mod resources;

/// The prelude should be all you need to use this crate!
/// Of course, if you want, you can also selectively use only the parts you need.
pub mod prelude {
    pub use bevy_ineffable_macros::ineff;
    pub use bevy_ineffable_macros::InputAction;

    pub use crate::bindings::ContinuousBinding;
    pub use crate::bindings::DualAxisBinding;
    pub use crate::bindings::InputBinding;
    pub use crate::bindings::PulseBinding;
    pub use crate::bindings::SingleAxisBinding;
    pub use crate::commands::IneffableCommands;
    pub use crate::config::InputConfig;
    pub use crate::input_action::InputAction;
    pub use crate::input_action::InputKind;
    pub use crate::plugin::IneffablePlugin;
    pub use crate::register::InputActionRegistrar;
    pub use crate::resources::Ineffable;
}
