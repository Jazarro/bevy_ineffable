use std::marker::PhantomData;

use bevy::log::error;
use bevy::prelude::Reflect;
use serde::{Deserialize, Serialize};

use crate::bindings::{AnalogInput, Chord, ChordLike, InputBinding, PulseBinding};
use crate::phantom::{IBWrp, Pulse, SingleAxis};

#[derive(Debug, Default, Serialize, Deserialize, Reflect, Clone, PartialEq)]
pub enum SingleAxisBinding {
    #[default]
    Dummy,
    Analog {
        input: AnalogInput,
        #[serde(default)]
        inversion: Inversion,
        #[serde(default)]
        sensitivity: Sensitivity,
    },
    Hold(Chord, Chord),
    Toggle(PulseBinding, PulseBinding),
}

#[derive(Debug, Default, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub enum Inversion {
    #[default]
    NotInverted,
    Inverted,
}

impl Inversion {
    pub fn maybe_invert(&self, value: f32) -> f32 {
        match self {
            Inversion::NotInverted => value,
            Inversion::Inverted => -1.0 * value,
        }
    }
    pub fn multiplier(&self) -> f32 {
        match self {
            Inversion::NotInverted => 1.0,
            Inversion::Inverted => -1.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
pub struct Sensitivity(f32);

impl Sensitivity {
    pub fn multiplier(&self) -> f32 {
        self.0
    }
}

impl Default for Sensitivity {
    fn default() -> Self {
        Sensitivity(1.0)
    }
}

// =====================================================================================================================
// ===== Builder stuff:
// =====================================================================================================================

impl SingleAxisBinding {
    /// Creates and returns a new builder for a single axis analog binding.
    ///
    /// `Analog` takes a single analog input that returns a direction as an `f32`.
    pub fn analog(input: AnalogInput) -> SingleAxisAnalogBuilder {
        SingleAxisAnalogBuilder::new(input)
    }
    /// Creates and returns a new builder for a single axis hold binding.
    ///
    /// `Hold` takes two binary inputs (negative and positive) that set the axis output to -1 or 1 as long as
    /// exactly one of them is active.
    #[must_use]
    pub fn hold() -> SingleAxisHoldBuilder {
        SingleAxisHoldBuilder::default()
    }
    /// Creates and returns a new builder for a single axis toggle binding.
    ///
    /// `Toggle` takes two pulse inputs (negative and positive) that toggle the axis output between 0, -1 and 1 when
    /// they pulse.
    #[must_use]
    pub fn toggle() -> SingleAxisToggleBuilder {
        SingleAxisToggleBuilder::default()
    }
}

#[derive(Debug)]
pub struct SingleAxisAnalogBuilder {
    input: AnalogInput,
    inversion: Inversion,
    sensitivity: Sensitivity,
}

impl SingleAxisAnalogBuilder {
    #[must_use]
    pub fn new(input: AnalogInput) -> Self {
        Self {
            input,
            inversion: Inversion::default(),
            sensitivity: Sensitivity::default(),
        }
    }
    #[must_use]
    pub fn invert(mut self) -> Self {
        self.inversion = if matches!(self.inversion, Inversion::Inverted) {
            Inversion::NotInverted
        } else {
            Inversion::Inverted
        };
        self
    }
    #[must_use]
    pub fn set_sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity.0 = sensitivity;
        self
    }
    #[must_use]
    pub fn build(self) -> IBWrp<SingleAxis> {
        let binding = InputBinding::SingleAxis(SingleAxisBinding::Analog {
            input: self.input,
            inversion: self.inversion,
            sensitivity: self.sensitivity,
        });
        IBWrp::<SingleAxis>(binding, PhantomData)
    }
}

#[derive(Debug, Default)]
pub struct SingleAxisHoldBuilder {
    negative: Option<Chord>,
    positive: Option<Chord>,
}

impl SingleAxisHoldBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn set_negative(mut self, input: impl Into<ChordLike>) -> Self {
        self.negative = Some(input.into().into());
        self
    }
    #[must_use]
    pub fn set_positive(mut self, input: impl Into<ChordLike>) -> Self {
        self.positive = Some(input.into().into());
        self
    }
    #[must_use]
    pub fn build(self) -> IBWrp<SingleAxis> {
        let binding = InputBinding::SingleAxis(SingleAxisBinding::Hold(
            self.negative.unwrap_or_default(),
            self.positive.unwrap_or_default(),
        ));
        IBWrp::<SingleAxis>(binding, PhantomData)
    }
}

#[derive(Debug, Default)]
pub struct SingleAxisToggleBuilder {
    negative: Option<PulseBinding>,
    positive: Option<PulseBinding>,
}

impl SingleAxisToggleBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn set_negative(mut self, input: IBWrp<Pulse>) -> Self {
        self.negative = Some(Self::unwrap_pulse(input));
        self
    }
    #[must_use]
    pub fn set_positive(mut self, input: IBWrp<Pulse>) -> Self {
        self.positive = Some(Self::unwrap_pulse(input));
        self
    }
    #[must_use]
    pub fn build(self) -> IBWrp<SingleAxis> {
        let binding = InputBinding::SingleAxis(SingleAxisBinding::Toggle(
            self.negative.unwrap_or(PulseBinding::Dummy),
            self.positive.unwrap_or(PulseBinding::Dummy),
        ));
        IBWrp::<SingleAxis>(binding, PhantomData)
    }
    fn unwrap_pulse(input: IBWrp<Pulse>) -> PulseBinding {
        if let InputBinding::Pulse(pulse) = input.0 {
            pulse
        } else {
            // Because of the wrapper containing the PhantomData, this should normally never happen (because it
            // would be a compile error), but log this just in case someone misunderstood how to use the builder.
            error!("SingleAxisBinding::Toggle requires a pulse input: every time it pulses, the axis input switches direction.\n\
            \tWe didn't get a pulse input, so this binding is not going to work.\n\
            \tTry to call this function like this:\n\
            \t`SingleAxisBinding::toggle().with_negative(PulseBinding::just_pressed(KeyCode::Left)).build()`");
            PulseBinding::Dummy
        }
    }
}
