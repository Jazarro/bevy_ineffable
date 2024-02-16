use bevy::prelude::Reflect;
use serde::{Deserialize, Serialize};

use crate::bindings::{BinaryInput, ContinuousBinding, PulseBinding, SingleAxisBinding};
use crate::input_action::InputKind;

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
pub enum InputBinding {
    SingleAxis(SingleAxisBinding),
    DualAxis {
        x: SingleAxisBinding,
        y: SingleAxisBinding,
    },
    Continuous(ContinuousBinding),
    Pulse(PulseBinding),
}

impl InputBinding {
    #[must_use]
    pub fn kind(&self) -> InputKind {
        match self {
            InputBinding::SingleAxis(_) => InputKind::SingleAxis,
            InputBinding::DualAxis { .. } => InputKind::DualAxis,
            InputBinding::Continuous(_) => InputKind::Continuous,
            InputBinding::Pulse(_) => InputKind::Pulse,
        }
    }
}

// =====================================================================================================================
// ===== ChordLike and From implementations
// ===== Used for builder pattern
// =====================================================================================================================

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
pub enum ChordLike {
    Single(BinaryInput),
    Multiple(Vec<BinaryInput>),
}

impl From<ChordLike> for Vec<BinaryInput> {
    fn from(value: ChordLike) -> Self {
        match value {
            ChordLike::Single(input) => vec![input],
            ChordLike::Multiple(inputs) => inputs,
        }
    }
}

impl<A: Into<BinaryInput>> From<A> for ChordLike {
    fn from(a: A) -> Self {
        ChordLike::Single(a.into())
    }
}

impl<A, B> From<(A, B)> for ChordLike
where
    A: Into<BinaryInput>,
    B: Into<BinaryInput>,
{
    fn from((a, b): (A, B)) -> Self {
        ChordLike::Multiple(vec![a.into(), b.into()])
    }
}

impl<A, B, C> From<(A, B, C)> for ChordLike
where
    A: Into<BinaryInput>,
    B: Into<BinaryInput>,
    C: Into<BinaryInput>,
{
    fn from((a, b, c): (A, B, C)) -> Self {
        ChordLike::Multiple(vec![a.into(), b.into(), c.into()])
    }
}

impl<A, B, C, D> From<(A, B, C, D)> for ChordLike
where
    A: Into<BinaryInput>,
    B: Into<BinaryInput>,
    C: Into<BinaryInput>,
    D: Into<BinaryInput>,
{
    fn from((a, b, c, d): (A, B, C, D)) -> Self {
        ChordLike::Multiple(vec![a.into(), b.into(), c.into(), d.into()])
    }
}

impl<A, B, C, D, E> From<(A, B, C, D, E)> for ChordLike
where
    A: Into<BinaryInput>,
    B: Into<BinaryInput>,
    C: Into<BinaryInput>,
    D: Into<BinaryInput>,
    E: Into<BinaryInput>,
{
    fn from((a, b, c, d, e): (A, B, C, D, E)) -> Self {
        ChordLike::Multiple(vec![a.into(), b.into(), c.into(), d.into(), e.into()])
    }
}
