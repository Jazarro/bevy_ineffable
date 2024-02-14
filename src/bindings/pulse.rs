use std::marker::PhantomData;
use std::time::Duration;

use bevy::prelude::Reflect;
use serde::{Deserialize, Serialize};

use crate::bindings::{Chord, ChordLike, InputBinding};
use crate::config::DurationInMillis;
use crate::phantom::{IBWrp, Pulse};

#[derive(Debug, Default, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub enum PulseBinding {
    #[default]
    Dummy,
    JustPressed(Chord),
    JustReleased(Chord),
    DoubleClick(Chord),
    Sequence(DurationInMillis, Vec<Chord>),
}

impl PulseBinding {
    pub fn just_pressed(input: impl Into<ChordLike>) -> IBWrp<Pulse> {
        let binding = InputBinding::Pulse(PulseBinding::JustPressed(input.into().into()));
        IBWrp::<Pulse>(binding, PhantomData)
    }
    pub fn just_released(input: impl Into<ChordLike>) -> IBWrp<Pulse> {
        let binding = InputBinding::Pulse(PulseBinding::JustReleased(input.into().into()));
        IBWrp::<Pulse>(binding, PhantomData)
    }
    pub fn double_click(input: impl Into<ChordLike>) -> IBWrp<Pulse> {
        let binding = InputBinding::Pulse(PulseBinding::DoubleClick(input.into().into()));
        IBWrp::<Pulse>(binding, PhantomData)
    }
    pub fn sequence(input: impl Into<ChordLike>) -> SequenceBuilder {
        SequenceBuilder::new(input)
    }
}

#[derive(Debug)]
pub struct SequenceBuilder {
    chords: Vec<Chord>,
}

impl SequenceBuilder {
    #[must_use]
    pub fn new(input: impl Into<ChordLike>) -> Self {
        SequenceBuilder {
            chords: vec![input.into().into()],
        }
    }
    #[must_use]
    pub fn followed_by(mut self, input: impl Into<ChordLike>) -> Self {
        self.chords.push(input.into().into());
        self
    }
    #[must_use]
    pub fn with_timing(self, timing: Duration) -> IBWrp<Pulse> {
        let binding = InputBinding::Pulse(PulseBinding::Sequence(
            timing.as_millis() as u64,
            self.chords,
        ));
        IBWrp::<Pulse>(binding, PhantomData)
    }
}
