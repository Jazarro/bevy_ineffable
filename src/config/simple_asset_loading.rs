//! This module makes loading InputConfigs a one-liner. It handles every step of the InputConfig loading:
//! - Scheduling InputConfigs to be loaded in the AssetServer
//! - Waiting for the configs to actually be loaded.
//! - Merging the configs and installing the merged result.
//!
//! If you want to do anything more fancy (like asset processing, hot reloading, or using a different asset loader
//! altogether), you are free to handle loading yourself and call `IneffableCommands.set_config()` directly.

use bevy::asset::{AssetServer, Assets, Handle, LoadState};
use bevy::prelude::{Commands, Res, Resource};

use crate::commands::IneffableCommands;
use crate::config::InputConfig;

#[derive(Debug, Resource)]
pub struct CurrentlyLoading {
    /// Ordered collection of handles.
    /// The first handle is for the base config, others are to be merged into the base in order.
    pub handles: Vec<Handle<InputConfig>>,
}

/// This system runs every tick as long as the CurrentlyLoading resource exists.
/// It waits for all the InputConfig asset handles to be loaded, then merges the configs and sets them.
pub(crate) fn manage_loading(
    mut commands: Commands<'_, '_>,
    mut ineffable: IneffableCommands<'_, '_>,
    handles: Res<'_, CurrentlyLoading>,
    assets: Res<'_, Assets<InputConfig>>,
    asset_server: Res<'_, AssetServer>,
) {
    let all_done = handles.handles.iter().all(|handle| {
        matches!(
            asset_server.load_state(handle),
            LoadState::Loaded | LoadState::Failed
        )
    });
    if !all_done {
        // Assets are not done loading yet. We're done here.
        return;
    }
    // Removing the CurrentlyLoading resource stops this system from getting called unnecessarily.
    // It also ensures that the assets are dropped from memory. (Unless the user keeps a strong handle for themself.)
    commands.remove_resource::<CurrentlyLoading>();

    let merged_config = handles
        .handles
        .iter()
        .filter_map(|handle| assets.get(handle))
        .fold(InputConfig::default(), |acc, next| acc.merge(next));
    ineffable.set_config(&merged_config);
}
