//! Contains a bevy plugin to help set up all the resources etc. needed by Ineffable.

use bevy::prelude::*;

use crate::bindings::*;
use crate::config::asset_loader_ron::InputConfigRonLoader;
use crate::config::simple_asset_loading::{manage_loading, CurrentlyLoading};
use crate::config::InputConfig;
use crate::processed::stateful::axis_dual::StatefulDualAxisBinding;
use crate::processed::stateful::axis_single::{
    StatefulSingleAxisBinding, StatefulSingleAxisBindingVariant,
};
use crate::processed::stateful::continuous::{
    StatefulContinuousBinding, StatefulContinuousBindingVariant,
};
use crate::processed::stateful::input_binary::StatefulBinaryInput;
use crate::processed::stateful::pulse::{StatefulPulseBinding, StatefulPulseBindingVariant};
use crate::processed::updating::update_input;
use crate::resources::ineffable_settings::IneffableSettings;
use crate::resources::meta_data::IneffableMetaData;
use crate::resources::Ineffable;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IneffablePlugin;

impl Plugin for IneffablePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Ineffable::default())
            .insert_resource(IneffableSettings::default())
            .insert_resource(IneffableMetaData::default())
            .init_asset::<InputConfig>()
            .init_asset_loader::<InputConfigRonLoader>()
            .add_systems(
                PreUpdate,
                (
                    manage_loading.run_if(resource_exists::<CurrentlyLoading>),
                    update_input,
                )
                    .chain(),
            );

        // TODO: Hide behind optional Reflect feature?
        app.register_type::<InputBinding>()
            .register_type::<DualAxisBinding>()
            .register_type::<SingleAxisBinding>()
            .register_type::<ContinuousBinding>()
            .register_type::<PulseBinding>()
            .register_type::<BinaryInput>()
            .register_type::<StatefulDualAxisBinding>()
            .register_type::<StatefulSingleAxisBinding>()
            .register_type::<StatefulContinuousBinding>()
            .register_type::<StatefulPulseBinding>()
            .register_type::<StatefulBinaryInput>()
            .register_type::<StatefulSingleAxisBindingVariant>()
            .register_type::<StatefulContinuousBindingVariant>()
            .register_type::<StatefulPulseBindingVariant>();
    }
}
