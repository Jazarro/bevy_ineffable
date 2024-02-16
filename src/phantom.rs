//! Types in this module are used to enforce compile time guarantees when interacting with
//! different InputKinds.

use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::bindings::InputBinding;
use crate::input_action::InputAction;

#[derive(
    Debug, Default, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct SingleAxis;

#[derive(
    Debug, Default, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct DualAxis;

#[derive(
    Debug, Default, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct Continuous;

#[derive(
    Debug, Default, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct Pulse;

/// A wrapper for `InputAction` that carries information about the `InputKind` it is.
///
/// The purpose of the wrapper is to provide compile-time guarantees that the `InputKind` is correct and you're not
/// accidentally using an `InputAction` in the wrong way.
///
/// # Examples
///
/// An instance can be obtained by using the `ineff!()` macro:
/// ```
/// # use bevy::prelude::KeyCode;
/// # use bevy_ineffable::phantom::{Continuous, IAWrp};
/// # use bevy_ineffable::prelude::*;
/// #[derive(InputAction)]
/// pub enum ExampleInput {
///      #[ineffable(continuous)]
///      Example,
/// }
/// let _: IAWrp<ExampleInput, Continuous> = ineff!(ExampleInput::Example);
/// ```
///
/// You typically shouldn't need to deal with this type directly.
/// However, if you ever need to unwrap it, do it like this:
/// ```
/// # use bevy::prelude::KeyCode;
/// # use bevy_ineffable::phantom::{Continuous, IAWrp};
/// # use bevy_ineffable::prelude::*;
/// # #[derive(InputAction)]
/// # pub enum ExampleInput {
/// #      #[ineffable(continuous)]
/// #      Example,
/// # }
/// let wrapped = ineff!(ExampleInput::Example);
/// let _: ExampleInput = wrapped.0;
/// ```
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct IAWrp<I: InputAction, Kind>(pub I, pub PhantomData<Kind>);

/// A wrapper for `InputBinding` that carries information about the `InputKind` it can be bound to.
///
/// This wrapper is used by the `InputConfigBuilder`, when creating an `InputConfig` programmatically. Its purpose is
/// to provide compile-time guarantees that the `InputKind` is correct and you're not accidentally using an
/// `InputAction` in the wrong way.
///
/// # Examples
///
/// An instance can be obtained by using one of the four builders:
/// ```
/// # use bevy::prelude::KeyCode;
/// # use bevy_ineffable::phantom::*;
/// # use bevy_ineffable::prelude::*;
/// let _ : IBWrp<DualAxis> = DualAxisBinding::builder().build();
/// let _ : IBWrp<SingleAxis> = SingleAxisBinding::hold().build();
/// let _ : IBWrp<Continuous> = ContinuousBinding::hold(KeyCode::Space);
/// let _ : IBWrp<Pulse> = PulseBinding::just_pressed(KeyCode::Space);
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IBWrp<Kind>(pub InputBinding, pub PhantomData<Kind>);
