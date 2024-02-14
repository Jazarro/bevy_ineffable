use std::time::Duration;

use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

use crate::bindings::InputBinding;
use crate::config::input_config::InputConfig;
use crate::config::DurationInMillis;
use crate::input_action::InputAction;
use crate::phantom::{IAWrp, IBWrp};

/// Builder to create an `InputConfig` programmatically.
///
/// Using the builder can be nice when prototyping, but it is recommended that in the final release, `InputConfig`
/// is loaded as an asset from a file.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InputConfigBuilder {
    double_click_timing: Option<DurationInMillis>,
    post_acceptance_delay: Option<DurationInMillis>,
    bindings: HashMap<String, HashMap<String, Vec<InputBinding>>>,
}

impl InputConfigBuilder {
    /// Create a new `InputConfigBuilder` using default settings.
    ///
    /// A more terse way of obtaining this would be to write:
    /// ```
    /// # use bevy_ineffable::config::builder::InputConfigBuilder;
    /// # use bevy_ineffable::config::InputConfig;
    /// let _: InputConfigBuilder = InputConfig::builder();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Double-click timing is the amount of time that can exist between the first and second inputs of a
    /// double-click action. The default timing used is 500 milliseconds (half a second), which is the same default
    /// that Microsoft uses in their operating system. Through this method, you can change it to any duration you like.
    #[must_use]
    pub fn double_click_timing(mut self, double_click_timing: Duration) -> Self {
        self.double_click_timing = Some(double_click_timing.as_millis() as u64);
        self
    }

    /// After detecting input, the system will ignore any further input for this amount of time.
    /// By default, the delay is turned off. Through this method, you can change it to any duration you like.
    #[must_use]
    pub fn post_acceptance_delay(mut self, post_acceptance_delay: Duration) -> Self {
        self.post_acceptance_delay = Some(post_acceptance_delay.as_millis() as u64);
        self
    }

    /// Bind an input method to an `InputAction`.
    ///
    ///  * `action`:  - Provide it like this: `ineff!(ExampleInput::Example)`
    ///  * `binding`: - Provide it by invoking the corresponding builder: e.g. `ContinuousBinding::hold(KeyCode::Space)`
    ///
    /// If the `InputKind`s of the action and the binding don't match, this will not compile.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy::prelude::KeyCode;
    /// # use bevy_ineffable::prelude::*;
    /// #[derive(InputAction)]
    /// pub enum ExampleInput {
    ///      #[ineffable(continuous)]
    ///      Example,
    /// }
    /// let _ = InputConfig::builder()
    ///     .bind(
    ///         ineff!(ExampleInput::Example),
    ///         ContinuousBinding::hold(KeyCode::ShiftLeft),
    ///     )
    ///     .build();
    /// ```
    #[must_use]
    pub fn bind<I: InputAction, Kind>(
        mut self,
        action: IAWrp<I, Kind>,
        binding: IBWrp<Kind>,
    ) -> Self {
        let group = self.bindings.entry(I::group_id().to_string()).or_default();
        let bindings = group.entry(action.0.action_id().to_string()).or_default();
        bindings.push(binding.0);
        self
    }

    /// Build a new `InputConfig` with the settings currently in the builder.
    /// This does not consume the builder: it can be re-used.
    #[must_use]
    pub fn build(&self) -> InputConfig {
        InputConfig {
            bindings: self.bindings.clone(),
            double_click_timing: self.double_click_timing,
            post_acceptance_delay: self.post_acceptance_delay,
        }
    }
}
