use std::fmt::Debug;

use bevy::prelude::KeyCode;
use serde::{Deserialize, Serialize};

use crate::bindings::{
    ContinuousBinding, DualAxisBinding, InputBinding, PulseBinding, SingleAxisBinding,
};

/// This trait represents an input-agnostic, abstract action that a player take.
/// For example, `Jump`, `Move`, or `Sprint`.
///
/// To start using this; create an enum and derive this trait. Each of the enum's variants is an `InputAction`.
/// You may derive this trait for multiple enums, if you wish to gather related actions into action groups.
///
/// The enum's variants must all be unit-like (don't contain other values), or it will not compile.
/// In addition, each variant must be annotated with exactly one of the following attributes, depending on the
/// kind of input it expects:
///
/// ```
/// # use bevy_ineffable_macros::InputAction;
/// # #[derive(InputAction)]
/// # pub enum Foo {
/// #[ineffable(dual_axis)] //<== dual_axis: returns a direction as a Vec2.
/// # A,
/// #[ineffable(single_axis)] //<== single_axis: returns a direction as an f32.
/// # B,
/// #[ineffable(continuous)] //<== continuous: returns true as long as the input is active.
/// # C,
/// #[ineffable(pulse)] //<== pulse: returns true for one tick when the input activates.
/// # D,
/// # }
/// ```
///
/// # Examples
/// ```
/// # use bevy_ineffable_macros::InputAction;
/// #[derive(InputAction)]
/// pub enum PlayerInput {
///     /// For example, can be bound to the WASD keys.
///     /// Returns a direction as a Vec2: e.g. (1,-1) if both the D and S keys are held down.
///     #[ineffable(dual_axis)]
///     Movement,
///     /// For example, can be bound to PageDown and PageUp keys.
///     /// Returns a direction as an f32: e.g. -1 if the PageDown key is pressed.
///     #[ineffable(single_axis)]
///     Rotation,
///     /// For example, can be bound to the Shift key.
///     /// Returns true as long as the key is held down.
///     #[ineffable(continuous)]
///     Sprint,
///     /// For example, can be bound to the left mouse button.
///     /// Returns true for one tick when the mouse is clicked.
///     #[ineffable(pulse)]
///     Shoot,
/// }
/// ```
pub trait InputAction {
    /// The name of the enum. Used to group together related `InputAction`s in the config file.
    fn group_id() -> &'static str
    where
        Self: Sized;
    /// The enum variant name.
    ///
    /// Only used when processing an `InputConfig`.
    fn action_id(&self) -> &'static str;
    /// The enum variant index.
    ///
    /// Internally, actions are stored in a Vec by index. This is used to look them up.
    fn index(&self) -> usize;
    /// What `InputKind` is this action?
    fn kind(&self) -> InputKind;
    /// An iterator over all the enum variants.
    ///
    /// Used internally when processing `InputConfig`s.
    fn iter() -> impl Iterator<Item = Self>
    where
        Self: Sized;
}

/// All `InputAction`s are separated into four different `InputKind`s. Each represents a slightly different way for
/// the player to interact with the game.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InputKind {
    /// A direction in a two-dimension plane, along two axes. When queried, returns a `Vec2`.
    ///
    /// For example: Movement controls in a first-person walker. It is commonly bound to `WASD`, which lets the player
    /// input a direction along two axes. Pressing both `A` and `W` at the same time returns `(-1, 1)`,
    /// or "move in the left-forward direction".
    DualAxis,
    /// A direction along a single axis. When queried, returns an `f32`.
    ///
    /// For example: Moving up or down flying controls: space to go up, shift to go down.
    SingleAxis,
    /// A binary (on or off) signal that can be active for an extended period of time.
    ///
    /// For example: a sprint button. As long as it's held down, sprint is active.
    /// As soon as the button is no longer held, the player stops sprinting.
    Continuous,
    /// An action that occasionally instantaneously happens. When queried, it returns true for exactly one tick.
    ///
    /// For example, shooting a revolver, jumping, initiating a conversation with an NPC.
    Pulse,
}

impl InputKind {
    /// Generates a short description of the `InputKind`. Used by the reporting module.
    #[must_use]
    pub(crate) fn explain(self) -> &'static str {
        match self {
            InputKind::SingleAxis => {
                "Indicates a direction along a single axis. Example: mouse wheel."
            }
            InputKind::DualAxis => "Indicates a direction along two axes. Example: joystick.",
            InputKind::Continuous => {
                "Binary signal, either on or off. Example: holding down the sprint button."
            }
            InputKind::Pulse => {
                "An instantaneous event. Example: clicking the mouse button to shoot."
            }
        }
    }
    //noinspection DuplicatedCode
    /// Creates a simple example `InputBinding` for each `InputKind`.
    /// This is used for offering suggestions to users in the logs.
    #[must_use]
    pub(crate) fn example(self) -> InputBinding {
        match self {
            InputKind::SingleAxis => {
                SingleAxisBinding::hold()
                    .set_negative(KeyCode::PageDown)
                    .set_positive(KeyCode::PageUp)
                    .build()
                    .0
            }
            InputKind::DualAxis => {
                DualAxisBinding::builder()
                    .set_x(
                        SingleAxisBinding::hold()
                            .set_negative(KeyCode::A)
                            .set_positive(KeyCode::D)
                            .build(),
                    )
                    .set_y(
                        SingleAxisBinding::hold()
                            .set_negative(KeyCode::S)
                            .set_positive(KeyCode::W)
                            .build(),
                    )
                    .build()
                    .0
            }
            InputKind::Continuous => ContinuousBinding::hold(KeyCode::ShiftLeft).0,
            InputKind::Pulse => PulseBinding::just_pressed(KeyCode::E).0,
        }
    }
}
