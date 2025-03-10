//! Shows the basics of how to use the crate and demonstrates use-cases for the four different InputKinds.
//! (DualAxis, SingleAxis, Continuous, Pulse)

use bevy::app::{Startup, Update};
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_ineffable::config::simple_asset_loading::MergeMode;

use bevy_ineffable::prelude::*;

/// Player movement speed.
const SPEED: f32 = 100.0;
/// Speed at which the player is rotated.
/// Value is negative because it feels more natural.
const ROTATE_SPEED: f32 = -2.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Always add the IneffablePlugin.
        .add_plugins(IneffablePlugin)
        // Also register any InputAction enums you are using.
        .register_input_action::<PlayerInput>()
        .add_systems(Startup, init)
        .add_systems(
            Update,
            (
                player_movement,
                player_rotation,
                player_blushing,
                player_teleportation,
            ),
        )
        .run();
}

#[derive(Debug, Default, Component)]
pub struct Player;

/// Note that InputAction is derived for this enum.
/// Each of this enum's variants is now a type of input action that can be checked.
/// This is an abstraction over things like pressed keys, mouse buttons, etc.
#[derive(Debug, InputAction)]
pub enum PlayerInput {
    /// W A S D
    /// Moves the player around on the screen.
    #[ineffable(dual_axis)] //<== dual_axis: returns a direction as a Vec2.
    Movement,
    /// Left and Right arrow keys
    /// Rotates the player.
    #[ineffable(single_axis)] //<== single_axis: returns a direction as an f32.
    Rotate,
    /// Left Shift key
    /// Tints the player red.
    #[ineffable(continuous)] //<== continuous: returns true as long as the input is active.
    Blush,
    /// Space bar
    /// Teleports the player back in the direction of the center of the screen.
    #[ineffable(pulse)] //<== pulse: returns true for one tick when the input activates.
    Teleport,
}

// =====================================================================================================================
// ===== Setting up the game.
// =====================================================================================================================

/// Create the camera and player entities and load keybindings from a file.
pub fn init(
    mut commands: Commands,
    mut ineffable: IneffableCommands,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn(Camera2d);
    commands
        .spawn(Sprite {
            image: images.add(white_square()),
            ..default()
        })
        .insert(Player);
    // Load keybindings and register them in the Ineffable Resource.
    // Without this step, no input can be read.
    ineffable.load_configs(vec![(MergeMode::Base, "basics.input.ron")]);
}

// =====================================================================================================================
// ===== Your in-game systems that run every tick.
// ===== This is where the input is queried.
// =====================================================================================================================

/// Move the player. This is a DualAxis InputAction, which returns a Vec2.
/// The scalar components of this vector are between -1.0 and 1.0.
fn player_movement(
    bindings: Res<Ineffable>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();

    let movement_direction = bindings.direction_2d(ineff!(PlayerInput::Movement));
    transform.translation.x += movement_direction.x * time.delta_secs() * SPEED;
    transform.translation.y += movement_direction.y * time.delta_secs() * SPEED;
}

/// Rotate the player. This is a SingleAxis InputAction, which returns an f32 between -1.0 and 1.0.
/// While the player is holding the button, rotate them clockwise or counter-clockwise at a constant rate.
fn player_rotation(
    bindings: Res<Ineffable>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();

    let rotate_direction = bindings.direction_1d(ineff!(PlayerInput::Rotate));
    transform.rotate_z(rotate_direction * time.delta_secs() * ROTATE_SPEED);
}

/// Decide what colour tint the player should have, based on whether they are currently holding down the blush button.
/// This is a Continuous InputAction, which returns true as long as the button is held down.
fn player_blushing(bindings: Res<Ineffable>, mut query: Query<&mut Sprite, With<Player>>) {
    let mut sprite = query.single_mut();

    sprite.color = if bindings.is_active(ineff!(PlayerInput::Blush)) {
        // When blushing, return a reddish tint.
        Color::srgb(0.8, 0.4, 0.4)
    } else {
        // When not blushing, return a blue/greenish tint.
        Color::srgb(0.4, 0.8, 0.8)
    };
}

/// Check if the player wants to teleport.
/// This is a Pulse InputAction, which returns true for exactly one tick, whenever the player activates it.
fn player_teleportation(bindings: Res<Ineffable>, mut query: Query<&mut Transform, With<Player>>) {
    let mut transform = query.single_mut();

    if bindings.just_pulsed(ineff!(PlayerInput::Teleport)) {
        // Teleportation moves the player towards the center of the screen.
        transform.translation.x += transform.translation.x.signum() * -100.;
        transform.translation.y += transform.translation.y.signum() * -100.;
    }
}

// =====================================================================================================================
// ===== Only boring helper stuff below.
// =====================================================================================================================

/// Boring helper stuff. Create a white square Image, so we have something to show on screen.
#[must_use]
fn white_square() -> Image {
    Image::new_fill(
        Extent3d {
            width: 100,
            height: 100,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[255, 255, 255, 255],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    )
}
