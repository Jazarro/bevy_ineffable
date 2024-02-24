//! This module makes loading InputConfigs a one-liner. It handles every step of the InputConfig loading:
//! - Scheduling InputConfigs to be loaded in the AssetServer
//! - Waiting for the configs to actually be loaded.
//! - Merging the configs and installing the merged result.
//!
//! If you want to do anything more fancy (like asset processing, hot reloading, or using a different asset loader
//! altogether), you are free to handle loading yourself and call `IneffableCommands.set_config()` directly.

use bevy::asset::{AssetServer, Assets, Handle, LoadState};
use bevy::prelude::{Commands, Reflect, Res, Resource};
use serde::{Deserialize, Serialize};

use crate::commands::IneffableCommands;
use crate::config::InputConfig;

#[derive(Debug, Resource)]
pub struct CurrentlyLoading {
    /// Ordered collection of handles.
    /// The first handle is for the base config, others are to be merged into the base in order.
    pub handles: Vec<(MergeMode, Handle<InputConfig>)>,
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
    let all_done = handles.handles.iter().all(|(_, handle)| {
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
        .filter_map(|(merge_mode, handle)| assets.get(handle).map(|asset| (merge_mode, asset)))
        .fold(
            InputConfig::default(),
            |acc, (merge_mode, next)| match merge_mode {
                MergeMode::Base => next.clone(),
                MergeMode::Append => acc.merge_append(next),
                MergeMode::Replace => acc.merge_replace(next),
            },
        );
    ineffable.set_config(&merged_config);
}

/// Determines how two `InputConfig`s are merged together.
#[derive(Debug, Default, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub enum MergeMode {
    /// Discard everything, use the base config as the new starting point.
    Base,
    /// If the appending config contains a mapping for an action, all keybindings from that mapping are appended to
    /// the bindings defined in the base config.
    Append,
    /// If the replacing config contains a mapping for an action, those bindings are used instead of those in the
    /// base config. This means that if the replacing config maps to an empty array, then the action is unbound.
    ///
    /// This should be used in most cases. As a player, when I'm remapping my keys, I usually don't want bindings from
    /// the base config to also still be active. If I do want that, I can always just copy those bindings.
    #[default]
    Replace,
}
