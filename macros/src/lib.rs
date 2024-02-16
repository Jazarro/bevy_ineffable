#![forbid(unsafe_code)]
#![forbid(unstable_features)]
#![forbid(missing_docs)]
//! Defines macros used by the `bevy_ineffable` crate.
//! Users shouldn't have to depend directly on this crate.

use proc_macro::TokenStream;

use syn::DeriveInput;

use crate::function::process_ineff_function;
use crate::input_action::implement_input_action;

mod function;
mod input_action;

/// The heart of the `bevy_ineffable` crate. An `InputAction` is something that happens upon user interaction.
///
/// This can only be derived for an enum containing unit-like variants. The enum is considered an group of input
/// actions, each variant is an `InputAction`.
/// Each variant must have exactly one of the following attributes, depending on the kind of input it expects:
///
///     #[ineffable(dual_axis)] //<== dual_axis: returns a direction as a Vec2.
///     #[ineffable(single_axis)] //<== single_axis: returns a direction as an f32.
///     #[ineffable(continuous)] //<== continuous: returns true as long as the input is active.
///     #[ineffable(pulse)] //<== pulse: returns true for one tick when the input activates.
///
/// # Examples
///
/// ```ignore
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
#[proc_macro_derive(InputAction, attributes(ineffable))]
pub fn derive_input_action(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let token_stream = implement_input_action(&ast).unwrap_or_else(|err| err.to_compile_error());
    // println!("{}", token_stream);
    token_stream.into()
}

/// Wraps around `InputAction`s to provide strong compile-time guarantees about the `InputKind` you're using.
///
/// # Examples
/// ```ignore
/// ineff!(ExampleInput::Example)
/// ```
#[proc_macro]
pub fn ineff(item: TokenStream) -> TokenStream {
    process_ineff_function(item)
}
