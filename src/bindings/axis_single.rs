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
    Analog(AnalogInput, AxisInversion, Option<AxisOptions>),
    Hold(Chord, Chord),
    Toggle(PulseBinding, PulseBinding),
}

#[derive(Debug, Default, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub enum AxisInversion {
    #[default]
    NotInverted,
    Inverted,
}

// TODO:....
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub struct AxisOptions {
    // pub positive_low: f32,
    // pub negative_low: f32,
    // pub sensitivity: f32,
}

// TODO:...
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub struct AxisDeadZone {}

// =====================================================================================================================
// ===== Builder stuff:
// =====================================================================================================================

impl SingleAxisBinding {
    pub fn analog() {
        panic!("Yet to be implemented.");
    }
    #[must_use]
    pub fn hold() -> SingleAxisHoldBuilder {
        SingleAxisHoldBuilder::default()
    }
    #[must_use]
    pub fn toggle() -> SingleAxisToggleBuilder {
        SingleAxisToggleBuilder::default()
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
