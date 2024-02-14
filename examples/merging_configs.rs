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
        .add_systems(
            Update,
            print_on_load.run_if(resource_exists::<CurrentlyLoading>()),
        )
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

#[derive(Debug, Resource)]
struct CurrentlyLoading {
    handle_base_profile: Handle<InputConfig>,
    handle_custom_profile: Handle<InputConfig>,
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(CurrentlyLoading {
        handle_base_profile: asset_server.load("../examples/assets/profile_base.input.ron"),
        handle_custom_profile: asset_server.load("../examples/assets/profile_custom.input.ron"),
    });
}

fn print_on_load(
    mut commands: Commands,
    mut ineffable: IneffableCommands,
    handles: Res<CurrentlyLoading>,
    assets: Res<Assets<InputConfig>>,
) {
    // Unwrap the handles, early return if they are not loaded yet.
    let Some(base_profile) = assets.get(&handles.handle_base_profile) else {
        return;
    };
    let Some(custom_profile) = assets.get(&handles.handle_custom_profile) else {
        return;
    };
    // Both assets are loaded, remove the CurrentlyLoading resource.
    commands.remove_resource::<CurrentlyLoading>();

    // Combine both input configs.
    // Keybindings from the custom profile are used if available, otherwise those from the base profile are used.
    let merged_profile = base_profile.merge(custom_profile);
    ineffable.set_config(&merged_profile);
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
