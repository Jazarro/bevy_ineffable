use std::marker::PhantomData;

use bevy::log::error;
use bevy::prelude::Reflect;
use serde::{Deserialize, Serialize};

use crate::bindings::{Chord, ChordLike, InputBinding, PulseBinding};
use crate::phantom::{Continuous, IBWrp, Pulse};

#[derive(Debug, Default, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub enum ContinuousBinding {
    #[default]
    Dummy,
    Hold(Chord),
    Toggle(PulseBinding),
}

impl ContinuousBinding {
    pub fn hold(input: impl Into<ChordLike>) -> IBWrp<Continuous> {
        let binding = InputBinding::Continuous(ContinuousBinding::Hold(input.into().into()));
        IBWrp::<Continuous>(binding, PhantomData)
    }
    pub fn toggle(input: IBWrp<Pulse>) -> IBWrp<Continuous> {
        if let InputBinding::Pulse(pulse) = input.0 {
            let binding = InputBinding::Continuous(ContinuousBinding::Toggle(pulse));
            IBWrp::<Continuous>(binding, PhantomData)
        } else {
            // Because of the wrapper containing the PhantomData, this should normally never happen (because it
            // would be a compile error), but log this just in case someone misunderstood how to use the builder.
            error!("ContinuousBinding::Toggle requires a pulse input: every time it pulses, the continuous input turns on or off.\n\
            \tWe didn't get a pulse input, so this binding is not going to work.\n\
            \tTry to call this function like this:\n\
            \t`ContinuousBinding::toggle(PulseBinding::just_pressed(KeyCode::Space))`");
            IBWrp::<Continuous>(
                InputBinding::Continuous(ContinuousBinding::Dummy),
                PhantomData,
            )
        }
    }
}
