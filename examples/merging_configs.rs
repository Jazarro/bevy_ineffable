//! This example loads multiple config files as assets and merges them together.
//! As a game developer, you could provide a default, base config that describes the default keybindings.
//! Players could then create a new config file with their own keybinding profile, in which they override some or
//! all of the default keybindings. The game can then load both configs and merge them.

use bevy::app::{Startup, Update};
use bevy::prelude::*;

use bevy_ineffable::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Always add the IneffablePlugin.
        .add_plugins(IneffablePlugin)
        // Also register any InputAction enums you are using.
        .register_input_action::<Action>()
        .add_systems(Startup, init)
        .add_systems(Update, spell_casting)
        .run();
}

#[derive(Debug, InputAction)]
pub enum Action {
    /// In the base config, this is bound to Q.
    /// In the custom profile, this is not mapped. Because of this, the binding from the base config is used.
    #[ineffable(pulse)]
    Defend,
    /// In the base config, this is bound to W.
    /// In the custom profile, this is bound to S.
    /// Since the custom profile overrides the base config, the only binding for this action that works is S.
    #[ineffable(pulse)]
    Attack,
}

fn init(mut ineffable: IneffableCommands) {
    ineffable.load_configs(vec![
        "../examples/assets/profile_base.input.ron",
        "../examples/assets/profile_custom.input.ron", // Try commenting out this line!
    ]);
}

fn spell_casting(bindings: Res<Ineffable>) {
    // Is bound to Q.
    if bindings.just_pulsed(ineff!(Action::Defend)) {
        println!("You raise your shield.");
    }
    // Originally bound to W, but because of the custom profile override, is actually bound to S.
    if bindings.just_pulsed(ineff!(Action::Attack)) {
        println!("You swing your sword.");
    }
}
