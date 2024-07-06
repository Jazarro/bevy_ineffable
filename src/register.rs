//! Handles registering `InputAction`s. This should be done once per `InputAction` enum at the start of the game.

use bevy::app::App;
use bevy::log::{error, warn};

use crate::input_action::InputAction;
use crate::resources::meta_data::{IneffableMetaData, IneffableMetaItem};

pub trait InputActionRegistrar {
    fn register_input_action<I: InputAction>(&mut self) -> &mut Self;
}

impl InputActionRegistrar for App {
    /// Registers an enum that derives `InputAction`. Multiple enums can be registered.
    ///
    /// Ineffable needs this to perform various compile-time and runtime checks.
    /// It is not possible to use any InputAction that was not registered through this function first.
    ///
    /// # Examples
    /// ```
    /// # use bevy::prelude::App;
    /// # use bevy_ineffable::prelude::*;
    /// #[derive(InputAction)]
    /// pub enum ExampleInput { /* some actions here */ }
    /// App::new().register_input_action::<ExampleInput>();
    /// ```
    fn register_input_action<I: InputAction>(&mut self) -> &mut Self {
        if !self.world_mut().contains_resource::<IneffableMetaData>() {
            self.world_mut()
                .insert_resource(IneffableMetaData::default());
        }
        let mut resource = self
            .world_mut()
            .get_resource_mut::<IneffableMetaData>()
            .expect("Missing resource IneffableMetaData. Try adding the IneffablePlugin first.");
        if let Some(previously_registered_group) = resource.group(I::group_id()) {
            let conflicting_group = construct_variants_meta_data::<I>();
            if &conflicting_group == previously_registered_group {
                warn!(
                    "Tried to register an InputAction more than once. \
                You can safely remove the redundant call to `app.register_input_action::<{}>()`",
                    I::group_id()
                );
            } else {
                error!(
                    "Tried to register two different InputActions with the same name: `{}`.\n\
                \tEach InputAction enum must have a unique name.\n\
                \tThis is almost certainly a bug, and may result in input not being read properly.",
                    I::group_id()
                );
            }
            return self;
        }
        resource
            .map
            .insert(I::group_id(), construct_variants_meta_data::<I>());
        self
    }
}

fn construct_variants_meta_data<I: InputAction>() -> Vec<IneffableMetaItem> {
    I::iter()
        .map(|action| IneffableMetaItem {
            group_id: I::group_id().to_string(),
            action_id: action.action_id().to_string(),
            kind: action.kind(),
            index: action.index(),
        })
        .collect()
}
