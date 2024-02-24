//! Contains the `SystemParam` that systems can use to set `InputConfig`s.

use bevy::asset::{AssetPath, AssetServer};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, Res, ResMut};

use crate::config::simple_asset_loading::{CurrentlyLoading, MergeMode};
use crate::config::InputConfig;
use crate::prelude::Ineffable;
use crate::processed::bound_action::BoundAction;
use crate::processed::processor::{collect_inputs, validate};
use crate::reporting::InputConfigReport;
use crate::resources::ineffable_settings::IneffableSettings;
use crate::resources::meta_data::IneffableMetaData;

/// Use this as a system parameter to validate and set `InputConfig`s.
///
/// # Examples
///
/// ```
/// # use bevy_ineffable::config::InputConfig;
/// # use bevy_ineffable::prelude::IneffableCommands;
/// pub fn system(mut commands: IneffableCommands) {
///     let config = InputConfig::default();
///     // You can validate the config, to obtain a report:
///     let report = commands.validate(&config);
///     // You can set the config, which automatically generates a report,
///     // dumps it to the log and returns it:
///     let report = commands.set_config(&config);
///     // You can set the config silently, which does not perform
///     // validation and does not write to the log:
///     commands.set_config_silent(&config);
/// }
/// ```
#[allow(missing_debug_implementations)]
#[derive(SystemParam)]
pub struct IneffableCommands<'w, 's> {
    commands: Commands<'w, 's>,
    /// Contains information about action groups and `InputAction`s, and which `InputKind`s they are.
    /// Used to validate the config.
    meta_data: Res<'w, IneffableMetaData>,
    processed_actions: ResMut<'w, Ineffable>,
    settings: ResMut<'w, IneffableSettings>,
    asset_server: Res<'w, AssetServer>,
}

impl IneffableCommands<'_, '_> {
    /// Scan the given `InputConfig` and check for errors. Compiles and returns a report of all the problems it finds.
    ///
    /// This is most useful when loading and merging multiple configs. This function allows you to generate a report
    /// for each config, before merging them, and give feedback to your players specifically about their custom config.
    #[must_use]
    pub fn validate(&self, config: &InputConfig) -> InputConfigReport {
        validate(&self.meta_data, config)
    }

    /// Sets the new `InputConfig`.
    ///
    /// This overrides any `InputConfig`s that were set before. To use settings from multiple configs at the same time,
    /// first merge them and then set the merged config.
    ///
    /// This function also automatically generates a report and dumps it to the log. If you don't want it to do that,
    /// use `set_config_silent()` instead.
    pub fn set_config(&mut self, config: &InputConfig) -> InputConfigReport {
        let report = self.validate(config);
        report.dump_to_log();
        self.set_config_silent(config);
        report
    }

    /// Sets the new `InputConfig`.
    ///
    /// This overrides any `InputConfig`s that were set before. To use settings from multiple configs at the same time,
    /// first merge them and then set the merged config.
    ///
    /// Consider using `set_config()` instead, for it will warn you if you make a mistake with your
    /// keybinding configuration. This function will silently swallow any bugs.
    pub fn set_config_silent(&mut self, config: &InputConfig) {
        let helper = collect_inputs(&self.meta_data, config);
        self.processed_actions.groups = config
            .bindings
            .iter()
            .filter(|(group_id, _)| self.meta_data.group_exists(group_id))
            .map(|(group_id, group_data)| {
                let mut bound_actions = Vec::new();
                for meta in self.meta_data.actions(group_id) {
                    if let Some(action) = group_data.get(&meta.action_id) {
                        bound_actions.push(BoundAction::new(meta, action, &helper));
                    } else {
                        bound_actions.push(BoundAction::new(meta, &[], &helper));
                    }
                }
                (group_id.clone(), bound_actions)
            })
            .collect();
        self.settings.set(config);
    }
    pub fn load_configs<'a>(&mut self, mut paths: Vec<(MergeMode, impl Into<AssetPath<'a>>)>) {
        let handles = paths
            .drain(..)
            .map(|(merge_mode, path)| (merge_mode, self.asset_server.load(path)))
            .collect();
        self.commands.insert_resource(CurrentlyLoading { handles });
    }
}
