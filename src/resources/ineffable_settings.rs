use std::time::Duration;

use bevy::prelude::{Reflect, Res, Resource, Timer, TimerMode};
use bevy::time::Time;
use bevy::utils::default;
use serde::{Deserialize, Serialize};

use crate::config::{DurationInMillis, InputConfig};

/// The default double click timing: the maximum delay between the first and second clicks of a double-click action.
/// This is the same as the default value in Microsoft Windows.
const DEFAULT_DOUBLE_CLICK_TIMING: DurationInMillis = 500;

#[derive(Debug, Resource, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq)]
pub struct IneffableSettings {
    /// The maximum delay between the first and second clicks of a double-click action.
    pub double_click_timing: Duration,
    /// An accessibility setting.
    /// From [gameaccessibilityguidelines](https://gameaccessibilityguidelines.com/include-a-cool-down-period-post-acceptance-delay-of-0-5-seconds-between-inputs/):
    /// "Conditions such as Parkinsons, essential tremor and cerebral palsy can reduce likelyhood of defined single
    /// presses, with slippage or shakiness common. This can result in unintended multiple presses, which can be a
    /// significant issue if interacting is already a drawn out process. If aiming for elderly or motor impaired
    /// players, including a simple cooldown period where no further input is recognised for a short period
    /// afterwards can avoid this."
    pub post_acceptance_delay: Option<PostAcceptanceDelay>,
}

#[derive(Debug, Default, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq)]
pub struct PostAcceptanceDelay {
    /// The duration of the delay.
    /// This is set when processing the InputConfig and will not change over the lifetime of this struct.
    delay: Duration,
    /// If true, the post acceptance delay will activate starting the next tick.
    should_activate: bool,
    timer: Option<Timer>,
}

impl PostAcceptanceDelay {
    /// This can be called during the update phase to announce that user input was detected.
    /// This will start the post acceptance delay, starting the next game tick.
    pub(crate) fn input_detected(&mut self) {
        self.should_activate = true;
    }
    fn is_blocking_input(&self) -> bool {
        self.timer.is_some()
    }
    pub(crate) fn tick(&mut self, time: Res<'_, Time>) {
        if self.should_activate {
            self.timer = Some(Timer::new(self.delay, TimerMode::Once));
            self.should_activate = false;
        } else if let Some(timer) = &mut self.timer {
            timer.tick(time.delta());
            if timer.finished() {
                self.timer = None;
            }
        }
    }
}

impl Default for IneffableSettings {
    fn default() -> Self {
        Self {
            double_click_timing: Duration::from_millis(DEFAULT_DOUBLE_CLICK_TIMING),
            post_acceptance_delay: None,
        }
    }
}

impl IneffableSettings {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn set(&mut self, config: &InputConfig) {
        self.double_click_timing = Duration::from_millis(
            config
                .double_click_timing
                .unwrap_or(DEFAULT_DOUBLE_CLICK_TIMING),
        );
        self.post_acceptance_delay = config
            .post_acceptance_delay
            .filter(|millis| millis > &0)
            .map(|millis| PostAcceptanceDelay {
                delay: Duration::from_millis(millis),
                ..default()
            });
    }
    #[must_use]
    pub(crate) fn input_blocked_by_pad(&self) -> bool {
        if let Some(pad) = &self.post_acceptance_delay {
            pad.is_blocking_input()
        } else {
            false
        }
    }
}
