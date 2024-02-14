//! Demonstrates how to implement a `charged action`: something that charges for a while, then pulses when released.
//!
//! An example of a charged action is Mario's jump: pressing the space bar performs a low jump, but holding the
//! space bar for a while makes the character jump much higher.
//!
//! In this example, we'll make a square wiggle with some classic stretch and squash, wiggling more vigorously
//! if the action was charged for longer.

use std::f32::consts::TAU;
use std::time::Duration;

use bevy::app::{Startup, Update};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use bevy_ineffable::prelude::*;

/// The maximum number of milliseconds that a wiggle can be charged for.
/// If it is charged for longer than this, charge time is clamped to this value.
const MAX_CHARGE_TIME: u128 = 1000;
/// How often to bounce when doing the wiggle.
const NR_BOUNCES: f32 = 5.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Always add the IneffablePlugin.
        .add_plugins(IneffablePlugin)
        // Also register any InputAction enums you are using.
        .register_input_action::<PlayerInput>()
        .add_systems(Startup, init)
        .add_systems(Update, (player_wiggle,))
        .run();
}

#[derive(Debug, Default, Component)]
pub struct Player {
    wiggle_timer: Timer,
    wiggle_strength: f32,
}

#[derive(Debug, InputAction)]
pub enum PlayerInput {
    /// Hold space bar for up to a second, then release.
    /// Wiggles the player. Bigger amplitude if charged for longer.
    #[ineffable(continuous)]
    Wiggle,
}

// =====================================================================================================================
// ===== Setting up the game.
// =====================================================================================================================

/// Create the camera and player entities and bind input to the InputActions.
pub fn init(
    mut commands: Commands,
    mut ineffable: IneffableCommands,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: images.add(make_square()),
            ..default()
        })
        .insert(Player::default());
    // Load keybindings and register them in the Ineffable Resource.
    // Without this step, no input can be read.
    ineffable.set_config(
        &InputConfig::builder()
            .bind(
                ineff!(PlayerInput::Wiggle),
                ContinuousBinding::hold(KeyCode::Space),
            )
            .build(),
    );
}

/// Stretch and squash the player based on wiggle input.
fn player_wiggle(
    bindings: Res<Ineffable>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let (mut transform, mut player) = query.single_mut();

    if bindings.just_deactivated(ineff!(PlayerInput::Wiggle)) {
        // Wiggle was just released! Create a timer to start the wiggle!
        player.wiggle_timer = Timer::new(Duration::from_secs(2), TimerMode::Once);
    }
    let scale = if let Some(duration) = bindings.charge_time(ineff!(PlayerInput::Wiggle)) {
        // Wiggle is charging up.
        // Slowly squash the player, maxing out the squash effect after one second.
        player.wiggle_strength =
            duration.as_millis().min(MAX_CHARGE_TIME) as f32 / (MAX_CHARGE_TIME as f32 * 2.0);
        1.0 - player.wiggle_strength
    } else if !player.wiggle_timer.finished() {
        // A timer is running, so we're currently animating the wiggle.
        player.wiggle_timer.tick(time.delta());
        let cos_wave = (player.wiggle_timer.percent() * TAU * NR_BOUNCES).cos() * -1.;
        let amplitude_modifier = player.wiggle_timer.percent_left();
        let fading_cos_wave = cos_wave * amplitude_modifier;
        1.0 + fading_cos_wave * player.wiggle_strength
    } else {
        1.0
    };
    transform.scale = Vec3::new(scale, scale.recip(), 1.0);
}

/// Boring helper stuff. Create a white square Image, so we have something to show on screen.
#[must_use]
fn make_square() -> Image {
    Image::new_fill(
        Extent3d {
            width: 100,
            height: 100,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[100, 100, 200, 255],
        TextureFormat::Rgba8Unorm,
    )
}
