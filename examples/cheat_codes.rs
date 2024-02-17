//! Demonstrates how to use Sequences to implement cheat codes.
//! A sequence is a number of pulse inputs that must be activated in order.

use std::fs;
use std::path::PathBuf;

use bevy::app::{Startup, Update};
use bevy::prelude::*;

use bevy_ineffable::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Always add the IneffablePlugin.
        .add_plugins(IneffablePlugin)
        // Also register any InputAction enums you are using.
        .register_input_action::<CheatCodes>()
        .add_systems(Startup, init)
        .add_systems(Update, listen_for_cheat_codes)
        .run();
}

#[derive(Debug, InputAction)]
pub enum CheatCodes {
    /// Type the word: password
    #[ineffable(pulse)]
    Password,
    /// Type the konami code (UpUpDownDownLeftRightLeftRightBA)
    #[ineffable(pulse)]
    KonamiCode,
}

/// Create the camera and player entities and load keybindings from a file.
pub fn init(mut ineffable: IneffableCommands) {
    // Load keybindings and register them in the Ineffable Resource.
    // Without this step, no input can be read.
    ineffable.set_config(&load_keybindings_from_file());
}

/// Move the player. This is a DualAxis InputAction, which returns a Vec2.
/// The scalar components of this vector are between -1.0 and 1.0.
fn listen_for_cheat_codes(bindings: Res<Ineffable>) {
    if bindings.just_pulsed(ineff!(CheatCodes::Password)) {
        println!("Hello world! Password accepted.");
    }
    if bindings.just_pulsed(ineff!(CheatCodes::KonamiCode)) {
        println!("Something spectacular happens!");
    }
}

// =====================================================================================================================
// ====== Only boring helper stuff below.
// =====================================================================================================================

/// Loads the keybinding config from file.
/// In a real game, you'd probably want to load this as an asset or something.
#[must_use]
fn load_keybindings_from_file() -> InputConfig {
    let path = PathBuf::new()
        .join("examples/assets")
        .join("cheat_code.ron");
    let data = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Unable to read InputConfig file at path: {path:?}"));
    ron::de::from_str::<InputConfig>(&data).expect("Unable to deserialise InputConfig")
}
