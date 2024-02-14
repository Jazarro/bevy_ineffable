[![Crates.io](https://img.shields.io/crates/v/ineffable)](https://crates.io/crates/ineffable)
[![Downloads](https://img.shields.io/crates/d/ineffable)](https://crates.io/crates/ineffable)
[![Docs](https://docs.rs/ineffable/badge.svg)](https://docs.rs/ineffable/latest/ineffable/)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/jazarro/ineffable#license)

# Bevy Ineffable

A simple-to-use input manager for the Bevy game engine that empowers players and makes accessibility easy.

## Core tenets

1. Make accessibility easy.
    - _Players can create and share custom input configs. Configs can be merged at runtime._
    - _[Post acceptance delay][1] helps players with conditions like Parkinson's avoid unintended key presses._
    - _[Toggling continuous input][2] helps players who physically cannot hold down a button for long periods of time._
    - _[Macro support][3] coming soon._
2. Offer a unified, abstracted view of input.
    - _Games should be agnostic of specific input devices._
    - _No more manually gathering keyboard, mouse and gamepad input from multiple sources._
3. Never allow the game to crash, but provide clear and direct feedback when something goes wrong.
    - _Scans player-made keybinding configurations and composes reports that provide detailed feedback about any
      problems._
4. Recognise the existence of different kinds of input (axis, dual-axis, continuous and pulse), and leverage the type
   system to differentiate between them at compile time.
    - DualAxis: _Inputs a direction along two axes. E.g. an analog stick._
    - SingleAxis: _Inputs a direction along one axis. E.g. the mouse wheel._
    - Continuous: _Is active continuously; for example, while a button is held down._
    - Pulse: _Pulses occasionally; for example, when a button is pressed._

## Quickstart

```toml
[dependencies]
# Add bevy_ineffable as a dependency to your `Cargo.toml`
bevy_ineffable = "0.1.0"
```

```rust no_run
use bevy::prelude::*;
use bevy_ineffable::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Always add the IneffablePlugin:
        .add_plugins(IneffablePlugin)
        // Also register GooseInput as an InputAction:
        .register_input_action::<GooseInput>()
        .add_systems(Startup, init)
        .add_systems(Update, update)
        .run();
}

/// Define an enum with `InputAction`s: abstract actions that keys can be bound to.
#[derive(InputAction)]
pub enum GooseInput {
    /// For this example, the only thing the player can do is honk.
    /// We must define what kind of input each action is.
    /// Honking is something that happens instantaneously, so we'll define it as a pulse.
    #[ineffable(pulse)]
    Honk,

    // You can add more actions here...

}

/// Create a config that binds the space bar to the `Honk` action.
/// We use the builder pattern here, but you can also load configs as an asset.
fn init(mut ineffable: IneffableCommands) {
    let config = InputConfig::builder()
        .bind(
            ineff!(GooseInput::Honk),
            PulseBinding::just_pressed(KeyCode::Space),
        )
        .build();
    ineffable.set_config(&config);
}

/// Whenever the Honk action pulses (when the space bar is just pressed down), 
/// we write to the console.
fn update(ineffable: Res<Ineffable>) {
    if ineffable.just_pulsed(ineff!(GooseInput::Honk)) {
        println!("Honk!");
    }
}
```

# More examples

Mor examples can be found in the `examples/` directory. Each example is in its own file. Try out the first one by
running:

```shell
cargo run --example basics
```

## Compatible Bevy versions

| bevy   | bevy_ineffable |
|--------|----------------|
| 0.12.* | 0.1.0          |

## Roadmap

- Macros
- Full gamepad and mouse support
- Multiple sets of bindings for the same group. This could be used, for example, to have different keybindings for
  the same actions for different players.
- Different binding contexts. This would allow bindings that might otherwise conflict with each other, because they
  are used in different contexts.
- Recording and playing back input. Could be used for:
    - Macros
    - Automated testing scenarios, such as tests that validate that a puzzle is still solvable or a jump is still
      possible.
    - Streaming input to a remote server.
- Systems to help automatically create a new keybinding profile. Something that listens for key presses and creates
  a new config based on what it detects.
- Dynamic buttons prompts. Show what button the player must press on the screen, regardless of what keybindings the
  player has set for themselves.
- Tie-in to GUI?

## License

Ineffable is dual-licensed under either:

- `MIT` ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- `Apache 2.0` ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means that when using this crate in your game, you may choose which license to use.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


[1]: https://gameaccessibilityguidelines.com/include-a-cool-down-period-post-acceptance-delay-of-0-5-seconds-between-inputs/

[2]: https://gameaccessibilityguidelines.com/avoid-provide-alternatives-to-requiring-buttons-to-be-held-down/

[3]: https://gameaccessibilityguidelines.com/provide-a-macro-system/