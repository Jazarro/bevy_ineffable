use std::fmt::Debug;

use bevy::asset::Asset;
use bevy::reflect::Reflect;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

use crate::bindings::InputBinding;
use crate::config::builder::InputConfigBuilder;

/// Contains input settings and keybindings for the game.
///
/// Can be constructed programmatically using the builder, or loaded as an asset from a file.
///
/// It is recommended that the developer provides a config with sensible defaults, and allows players to create their
/// own configs in which they can selectively override those defaults. The base settings can then be merged with
/// the player-provided overrides, and the result offered to ineffable to use during gameplay.
#[derive(Debug, Default, Serialize, Deserialize, Asset, Reflect, Clone, PartialEq)]
pub struct InputConfig {
    #[serde(default)]
    pub double_click_timing: Option<DurationInMillis>,
    #[serde(default)]
    pub post_acceptance_delay: Option<DurationInMillis>,
    #[serde(default)]
    pub bindings: HashMap<String, HashMap<String, Vec<InputBinding>>>,
    // #[serde(default)]
    // pub macros: Vec<Macro>,
}

/// A simply type alias to make it clear what this integer represents; a duration in milliseconds.
pub type DurationInMillis = u64;

impl InputConfig {
    /// Create a new `InputConfig`, using default values.
    ///
    /// If you want to obtain a non-default InputConfig, try using the builder instead:
    /// ```
    /// # use bevy_ineffable::config::InputConfig;
    /// # let _ =
    /// InputConfig::builder()
    ///     // (...)
    ///     .build();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an `InputConfigBuilder`, which can be used to construct an `InputConfig`.
    #[must_use]
    pub fn builder() -> InputConfigBuilder {
        InputConfigBuilder::default()
    }

    /// Creates a new `InputConfig` that represents the result of merging together two existing configs.
    ///
    /// Neither of the given `InputConfig`s will be changed, this function creates a new instance of the struct.
    ///
    /// This config is considered the base; earlier in the load order. The `other` config is later in the load order
    /// and will (partially) override this one. Any InputActions or miscellaneous settings that are present in the
    /// `other` config will take precedence, otherwise it will fall back to the value in this one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::time::Duration;
    /// # use bevy_ineffable::prelude::*;
    /// // First config only has double-click-timing.
    /// let first = InputConfig::builder()
    ///     .double_click_timing(Duration::from_millis(10))
    ///     .build();
    /// assert_eq!(first.double_click_timing, Some(10));
    /// assert_eq!(first.post_acceptance_delay, None);
    ///
    /// // Second config only has post-acceptance-delay.
    /// let second = InputConfig::builder()
    ///     .post_acceptance_delay(Duration::from_millis(200))
    ///     .build();
    /// assert_eq!(second.double_click_timing, None);
    /// assert_eq!(second.post_acceptance_delay, Some(200));
    ///
    /// // First merge combines them and has both.
    /// let merge12 = first.merge(&second);
    /// assert_eq!(merge12.double_click_timing, Some(10));
    /// assert_eq!(merge12.post_acceptance_delay, Some(200));
    ///
    /// // Third has a different double-click-timing.
    /// let third = InputConfig::builder()
    ///     .double_click_timing(Duration::from_millis(42))
    ///     .build();
    /// assert_eq!(third.double_click_timing, Some(42));
    /// assert_eq!(third.post_acceptance_delay, None);
    ///
    /// // Upon merging third into merge12, there was a conflict.
    /// // Third wins, its values override those in merge12.
    /// let merge123 = merge12.merge(&third);
    /// assert_eq!(merge123.double_click_timing, Some(42));
    /// assert_eq!(merge123.post_acceptance_delay, Some(200));
    /// ```
    #[must_use]
    pub fn merge_replace(&self, other: &InputConfig) -> Self {
        self.merge_inner(other, false)
    }

    /// If the base and appending configs both define bindings for the same action, bindings are appended and all
    /// bindings end up in the final result. This is in contrast to the merge_replace() function, that replaces
    /// bindings in the base config, if the action is defined in the replace config.
    #[must_use]
    pub fn merge_append(&self, other: &InputConfig) -> Self {
        self.merge_inner(other, true)
    }

    /// Deprecated, kept (for now) for backwards compatibility.
    /// Replaced by merge_replace()
    #[must_use]
    pub fn merge(&self, other: &InputConfig) -> Self {
        self.merge_replace(other)
    }

    #[must_use]
    fn merge_inner(&self, other: &InputConfig, append: bool) -> Self {
        let mut value = self.clone();
        for (group_id, action_id, action) in other.bindings.iter().flat_map(|(group_id, group)| {
            group
                .iter()
                .map(move |(action_id, action)| (group_id, action_id, action))
        }) {
            let actions = value.bindings.entry(group_id.clone()).or_default();
            let bindings = actions.entry(action_id.clone()).or_default();
            if !append {
                bindings.clear();
            }
            bindings.append(&mut action.clone());
        }
        if other.post_acceptance_delay.is_some() {
            value.post_acceptance_delay = other.post_acceptance_delay;
        }
        if other.double_click_timing.is_some() {
            value.double_click_timing = other.double_click_timing;
        }
        value
    }
}
