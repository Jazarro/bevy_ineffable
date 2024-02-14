use std::time::Duration;

use bevy::prelude::{Reflect, Resource, Vec2};
use bevy::utils::HashMap;

use crate::input_action::InputAction;
use crate::phantom::{Continuous, DualAxis, IAWrp, Pulse, SingleAxis};
use crate::processed::bound_action::BoundAction;
use crate::processed::stateful::{axis_dual, axis_single, continuous, pulse};

/// Main entry point for querying the state of `InputAction`s.
///
/// There are four different kinds of `InputAction`s: dual-axis, single-axis, continuous, pulse.
/// This `Resource` contains different functions for querying each of them. To help you avoid accidentally using the
/// wrong action, the `ineff!()` macro provides a compile-time guarantee that the given `InputAction` is of the
/// correct `InputKind`.
///
/// # Examples
///
/// For example, this works:
///
/// ```
/// # use bevy::math::Vec2;
/// # use bevy_ineffable::prelude::*;
/// # let ineffable = Ineffable::default();
/// #[derive(InputAction)]
/// pub enum ExampleInput {
///
///      #[ineffable(dual_axis)]
///      DualAxisAction,
///
///      #[ineffable(single_axis)]
///      SingleAxisAction,
///
///      #[ineffable(continuous)]
///      ContinuousAction,
///
///      #[ineffable(pulse)]
///      PulseAction,
///
/// }
/// let _: Vec2 = ineffable.direction_2d(ineff!(ExampleInput::DualAxisAction));
/// let _: f32 = ineffable.direction_1d(ineff!(ExampleInput::SingleAxisAction));
/// let _: bool = ineffable.is_active(ineff!(ExampleInput::ContinuousAction));
/// let _: bool = ineffable.just_pulsed(ineff!(ExampleInput::PulseAction));
/// ```
///
/// In this next example, the given action is of the wrong kind, and this snippet does not compile.
/// The exact compile-time error you'll get is:
/// > expected `IAWrp<ExampleInput, DualAxis>`, but found `IAWrp<ExampleInput, SingleAxis>`
///
/// ```compile_fail
/// # use bevy::math::Vec2;
/// # use bevy_ineffable::prelude::*;
/// # let ineffable = Ineffable::default();
/// #[derive(InputAction)]
/// pub enum ExampleInput {
///      #[ineffable(single_axis)] // <== This is not a dual axis!
///      NotADualAxis,
/// }
/// // This does not compile!
/// let _: Vec2 = ineffable.direction_2d(ineff!(ExampleInput::NotADualAxis));
/// ```
#[derive(Debug, Default, Resource, Reflect, Clone)]
pub struct Ineffable {
    pub(crate) _contexts: HashMap<String, InputContext>,
    pub(crate) groups: HashMap<String, ProcessedBindingGroup>,
}

/// All bindings within a certain group. The Vec index is the order in which they appear in the enum.
type ProcessedBindingGroup = Vec<BoundAction>;
/// A context. Consists of a number of group names.
type InputContext = Vec<String>;

impl Ineffable {
    // =================================================================================================================
    // ===== Dual Axis
    // =================================================================================================================

    /// Returns a `Vec2` representing the 2-dimensional direction of the given dual-axis `InputAction`.
    ///
    /// Call like this: `ineffable.direction_2d(ineff!(ExampleInput::ExampleVariant))`
    pub fn direction_2d<I: InputAction>(&self, action: IAWrp<I, DualAxis>) -> Vec2 {
        axis_dual::bound_action(self, action)
            .map(|bound| bound.value)
            .unwrap_or_default()
    }

    // =================================================================================================================
    // ===== Single Axis
    // =================================================================================================================

    /// Returns a `f32` representing the 1-dimensional direction of the given single-axis `InputAction`.
    ///
    /// Call like this: `ineffable.direction_1d(ineff!(ExampleInput::ExampleVariant))`
    pub fn direction_1d<I: InputAction>(&self, action: IAWrp<I, SingleAxis>) -> f32 {
        axis_single::bound_action(self, action)
            .map(|bound| bound.value)
            .unwrap_or_default()
    }

    // =================================================================================================================
    // ===== Continuous
    // =================================================================================================================

    /// Returns true iff the given continuous action is currently considered active.
    ///
    /// Call like this: `ineffable.is_active(ineff!(ExampleInput::Example))`
    pub fn is_active<I: InputAction>(&self, action: IAWrp<I, Continuous>) -> bool {
        continuous::bound_action(self, action).is_some_and(|binding| binding.active)
    }

    /// Returns true iff the given continuous action is active now, but was not active last tick.
    ///
    /// Call like this: `ineffable.just_activated(ineff!(ExampleInput::ExampleVariant))`
    pub fn just_activated<I: InputAction>(&self, action: IAWrp<I, Continuous>) -> bool {
        continuous::bound_action(self, action)
            .is_some_and(|binding| binding.active && !binding.active_previous_tick)
    }

    /// Returns true iff the given continuous action was active last tick, but is not active anymore.
    ///
    /// Call like this: `ineffable.just_deactivated(ineff!(ExampleInput::ExampleVariant))`
    pub fn just_deactivated<I: InputAction>(&self, action: IAWrp<I, Continuous>) -> bool {
        continuous::bound_action(self, action)
            .is_some_and(|binding| !binding.active && binding.active_previous_tick)
    }

    /// Returns an optional `Duration` describing how long the given continuous action has been active for.
    ///
    /// Returns:
    /// - `Some(duration)` if this action is currently active.
    /// - `Some(duration)` if the action is not active, but was active last game tick.
    /// - `None` otherwise.
    ///
    /// Call like this: `ineffable.charge_time(ineff!(ExampleInput::ExampleVariant))`
    pub fn charge_time<I: InputAction>(&self, action: IAWrp<I, Continuous>) -> Option<Duration> {
        continuous::bound_action(self, action).and_then(|binding| binding.charging_duration())
    }

    // =================================================================================================================
    // ===== Pulse
    // =================================================================================================================

    /// Returns true iff the pulse action was activated this game tick. This only stays true for one tick.
    ///
    /// Call like this: `ineffable.just_pulsed(ineff!(ExampleInput::ExampleVariant))`
    pub fn just_pulsed<I: InputAction>(&self, action: IAWrp<I, Pulse>) -> bool {
        pulse::bound_action(self, action).is_some_and(|binding| binding.just_pulsed)
    }
}
