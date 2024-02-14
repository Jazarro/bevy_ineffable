use bevy::prelude::Reflect;

use crate::bindings::{BinaryInput, Chord};
use crate::processed::processor::Helper;
use crate::processed::updating::InputSources;
use crate::reporting::{ActionLocation, InputConfigProblem, InputConfigReport};

#[derive(Debug, Reflect, Clone)]
pub(crate) struct StatefulBinaryInput {
    pub(crate) binary_input: ProcessedChord,
    pub(crate) active: bool,
    pub(crate) active_previous_tick: bool,
    pub(crate) blocked: bool,
    /// These are the inputs that conflict with this one and take precedence over it.
    /// If any of these binary inputs is active, this one cannot be active.
    pub(crate) blockers: Vec<Chord>,
}

/// This enum prevents us requiring a Vec for every single input.
#[derive(Debug, Reflect, Clone)]
pub(crate) enum ProcessedChord {
    Dummy,
    Single(BinaryInput),
    Chord(Vec<BinaryInput>),
}

impl ProcessedChord {
    fn new(chord: Chord) -> Self {
        if chord.len() > 1 {
            ProcessedChord::Chord(chord)
        } else if let Some(input) = chord.first() {
            ProcessedChord::Single(input.clone())
        } else {
            ProcessedChord::Dummy
        }
    }
}

pub(crate) fn check_for_problems(
    input: &Chord,
    report: &mut InputConfigReport,
    loc: &ActionLocation,
) {
    for (index, first) in input.iter().enumerate() {
        for second in input.iter().skip(index + 1) {
            if first == second {
                report.warning(InputConfigProblem::ChordContainsDuplicates { loc: loc.clone() });
            }
        }
    }
}

impl StatefulBinaryInput {
    pub(crate) fn new(value: &Chord, helper: &Helper<'_>) -> StatefulBinaryInput {
        let blockers = helper
            .inputs
            .iter()
            .filter_map(|(_, other)| {
                if is_blocked_by(value, other) {
                    Some(other.clone())
                } else {
                    None
                }
            })
            .collect();
        StatefulBinaryInput {
            binary_input: ProcessedChord::new(value.clone()),
            active: false,
            active_previous_tick: false,
            blocked: false,
            blockers,
        }
    }
    pub(crate) fn is_active(&self) -> bool {
        !self.blocked && self.active
    }
    pub(crate) fn just_pressed(&self) -> bool {
        !self.blocked && self.active && !self.active_previous_tick
    }
    pub(crate) fn just_released(&self) -> bool {
        !self.blocked && !self.active && self.active_previous_tick
    }
    pub(crate) fn update(&mut self, sources: &mut InputSources<'_>) {
        self.active_previous_tick = self.active;

        // If the post-acceptance-delay is active, then do nothing. We should ignore all user input.
        if sources.settings.input_blocked_by_pad() {
            return;
        }

        self.active = Self::is_chord_pressed(&self.binary_input, sources);
        self.blocked = self
            .blockers
            .iter()
            .any(|blocker| blocker.iter().all(|child| Self::is_pressed(child, sources)));

        // If the user just activated this input, then report this to the post-acceptance-delay.
        if self.just_pressed() {
            if let Some(pad) = &mut sources.settings.post_acceptance_delay {
                pad.input_detected();
            }
        }
    }
    fn is_chord_pressed(chord: &ProcessedChord, sources: &InputSources<'_>) -> bool {
        match chord {
            ProcessedChord::Dummy => false,
            ProcessedChord::Single(input) => Self::is_pressed(input, sources),
            ProcessedChord::Chord(inputs) => {
                inputs.iter().all(|child| Self::is_pressed(child, sources))
            }
        }
    }
    fn is_pressed(input: &BinaryInput, sources: &InputSources<'_>) -> bool {
        match input {
            BinaryInput::Key(key_code) => sources.keys.pressed(*key_code),
            BinaryInput::MouseButton(mouse_btn) => sources.mouse_buttons.pressed(*mouse_btn),
        }
    }
}

fn is_blocked_by(this: &Chord, other: &Chord) -> bool {
    if this.is_empty() || this.len() >= other.len() {
        // If this is empty, blockers don't matter because it will never activate anyways.
        // Also, other must be longer in order to block this,
        // because only a more specific chord (direct superset) can block another chord.
        return false;
    }
    this.iter().all(|this_input| other.contains(this_input))
}
