use bevy::prelude::{KeyCode, MouseButton, Reflect};
use serde::{Deserialize, Serialize};

/// A chord is a set of unique inputs that have to be activated at the same time.
/// Example: Ctrl-S to save a document.
/// An empty chord is considered a dummy, it will never activate.
pub type Chord = Vec<BinaryInput>;

/// Something that can provide a discrete, binary signal: on or off.
/// Example: a button that can either be pressed down or not.
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub enum BinaryInput {
    Key(KeyCode),
    // KeyGroup(KeyGroup),
    MouseButton(MouseButton),
    // MouseWheel(),
    // MouseMotion(),
    // GamePadButton,
    // AnalogStick,
    // Etc,
}

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
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

impl From<MouseButton> for BinaryInput {
    fn from(input: MouseButton) -> Self {
        BinaryInput::MouseButton(input)
    }
}

impl From<KeyCode> for BinaryInput {
    fn from(input: KeyCode) -> Self {
        BinaryInput::Key(input)
    }
}
