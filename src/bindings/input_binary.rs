use std::slice::Iter;

use bevy::prelude::{GamepadButtonType, KeyCode, MouseButton, Reflect, ScanCode};
use serde::{Deserialize, Serialize};

use crate::bindings::input_analog::AnalogInput;

/// A chord is a set of unique inputs that have to be activated at the same time.
/// Example: Ctrl-S to save a document.
/// An empty chord is considered a dummy, it will never activate.
pub type Chord = Vec<BinaryInput>;

/// Something that can provide a discrete, binary signal: on or off.
/// Example: a button that can either be pressed down or not.
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
pub enum BinaryInput {
    Key(KeyCode),
    ScanCode(ScanCode),
    KeyGroup(KeyGroup),
    MouseButton(MouseButton),
    Gamepad(GamepadButtonType),
    /// A binary input taken from an analog axis.
    /// If the axis passes a given threshold, it is considered active, otherwise it is not.
    /// For example; pushing a game pad's left trigger to the left counting as a button press.
    Axis(AnalogInput, Threshold),
}

/// Used to convert an analog axis input to a binary input.
/// Given an analog input (represented by an f32) at what threshold should the binary input become active?
/// If the given `f32`-threshold is positive, the analog value must be equal or greater than the threshold to activate the input.
/// If the given `f32`-threshold is negative, the analog value must be equal or smaller than the threshold to activate the input.
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
pub struct Threshold(pub f32);

impl Threshold {
    /// Create a new Threshold with the given value.
    pub fn new(value: f32) -> Self {
        Self(value)
    }
    /// Create a new Threshold at a sensible preset value of magnitude 0.75.
    /// This is for inputs that are 'down', ie: towards negative. Same as `preset_neg()`.
    pub fn preset_down() -> Self {
        Self::preset_neg()
    }
    /// Create a new Threshold at a sensible preset value of magnitude 0.75.
    /// This is for inputs that are 'left', ie: towards negative. Same as `preset_neg()`.
    pub fn preset_left() -> Self {
        Self::preset_neg()
    }
    /// Create a new Threshold at a sensible preset value of magnitude 0.75.
    /// This is for inputs that are towards negative.
    pub fn preset_neg() -> Self {
        Self(-0.75)
    }
    /// Create a new Threshold at a sensible preset value of magnitude 0.75.
    /// This is for inputs that are 'up', ie: towards positive. Same as `preset_pos()`.
    pub fn preset_up() -> Self {
        Self::preset_pos()
    }
    /// Create a new Threshold at a sensible preset value of magnitude 0.75.
    /// This is for inputs that are 'right', ie: towards positive. Same as `preset_pos()`.
    pub fn preset_right() -> Self {
        Self::preset_pos()
    }
    /// Create a new Threshold at a sensible preset value of magnitude 0.75.
    /// This is for inputs that are towards positive.
    pub fn preset_pos() -> Self {
        Self(0.75)
    }

    /// Returns true iff the threshold is reached.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_ineffable::bindings::Threshold;
    /// assert!(Threshold::new(-1.0).is_reached(-2.0));
    /// assert!(Threshold::new(-1.0).is_reached(-1.0));
    /// assert!(!Threshold::new(-1.0).is_reached(-0.5));
    /// assert!(!Threshold::new(-1.0).is_reached(2.0));
    ///
    /// assert!(Threshold::new(1.0).is_reached(2.0));
    /// assert!(Threshold::new(1.0).is_reached(1.0));
    /// assert!(!Threshold::new(1.0).is_reached(0.5));
    /// assert!(!Threshold::new(1.0).is_reached(-2.0));
    /// ```
    pub fn is_reached(&self, value: f32) -> bool {
        if self.0.is_sign_negative() {
            value <= self.0
        } else {
            self.0 <= value
        }
    }
}

/// Keys that are found in multiple locations on the keyboard.
///
/// For example, let's say you want to use any Enter to perform an action, whether it is the main one or the
/// numpad one. You want the player to be able to use them interchangeably. You could make two bindings, or you could
/// use `KeyGroup::Enter`, which matches both.
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Eq, Hash)]
pub enum KeyGroup {
    /// Matches both `KeyCode::Return` and `KeyCode::NumpadEnter`.
    Enter,
    /// Matches either of the control keys.
    Control,
    /// Matches either of the shift keys.
    Shift,
    /// Matches either of the alt keys.
    Alt,
    /// Matches either of the super keys.
    ///
    /// Generic keyboards usually display these keys with the *Microsoft Windows* logo.
    /// Apple keyboards call this key the *Command Key* and display it using the ⌘ character.
    Super,
    /// Number zero from the top row or the numpad.
    Number0,
    /// Number one from the top row or the numpad.
    Number1,
    /// Number two from the top row or the numpad.
    Number2,
    /// Number three from the top row or the numpad.
    Number3,
    /// Number four from the top row or the numpad.
    Number4,
    /// Number five from the top row or the numpad.
    Number5,
    /// Number six from the top row or the numpad.
    Number6,
    /// Number seven from the top row or the numpad.
    Number7,
    /// Number eight from the top row or the numpad.
    Number8,
    /// Number nine from the top row or the numpad.
    Number9,
}

impl KeyGroup {
    pub fn iter(&self) -> Iter<'_, KeyCode> {
        match self {
            KeyGroup::Enter => [KeyCode::Return, KeyCode::NumpadEnter].iter(),
            KeyGroup::Control => [KeyCode::ControlLeft, KeyCode::ControlRight].iter(),
            KeyGroup::Shift => [KeyCode::ShiftLeft, KeyCode::ShiftRight].iter(),
            KeyGroup::Alt => [KeyCode::AltLeft, KeyCode::AltRight].iter(),
            KeyGroup::Super => [KeyCode::SuperLeft, KeyCode::SuperRight].iter(),
            KeyGroup::Number0 => [KeyCode::Key0, KeyCode::Numpad0].iter(),
            KeyGroup::Number1 => [KeyCode::Key1, KeyCode::Numpad1].iter(),
            KeyGroup::Number2 => [KeyCode::Key2, KeyCode::Numpad2].iter(),
            KeyGroup::Number3 => [KeyCode::Key3, KeyCode::Numpad3].iter(),
            KeyGroup::Number4 => [KeyCode::Key4, KeyCode::Numpad4].iter(),
            KeyGroup::Number5 => [KeyCode::Key5, KeyCode::Numpad5].iter(),
            KeyGroup::Number6 => [KeyCode::Key6, KeyCode::Numpad6].iter(),
            KeyGroup::Number7 => [KeyCode::Key7, KeyCode::Numpad7].iter(),
            KeyGroup::Number8 => [KeyCode::Key8, KeyCode::Numpad8].iter(),
            KeyGroup::Number9 => [KeyCode::Key9, KeyCode::Numpad9].iter(),
        }
    }
}

// =====================================================================================================================
// ===== From implementations: required for builder pattern
// =====================================================================================================================

impl From<KeyCode> for BinaryInput {
    fn from(input: KeyCode) -> Self {
        BinaryInput::Key(input)
    }
}

impl From<ScanCode> for BinaryInput {
    fn from(input: ScanCode) -> Self {
        BinaryInput::ScanCode(input)
    }
}

impl From<KeyGroup> for BinaryInput {
    fn from(input: KeyGroup) -> Self {
        BinaryInput::KeyGroup(input)
    }
}

impl From<MouseButton> for BinaryInput {
    fn from(input: MouseButton) -> Self {
        BinaryInput::MouseButton(input)
    }
}

impl From<GamepadButtonType> for BinaryInput {
    fn from(input: GamepadButtonType) -> Self {
        BinaryInput::Gamepad(input)
    }
}
