use crate::Action;
use crate::KeyChord;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mapping {
    key_chords: Vec<KeyChord>,
    action: Action,
}

impl Mapping {
    pub fn new(key_chords: Vec<KeyChord>, action: Action) -> Mapping {
        Mapping { key_chords, action }
    }

    pub fn get_key_chords(&self) -> &Vec<KeyChord> {
        &self.key_chords
    }

    pub fn get_action(&self) -> &Action {
        &self.action
    }

    pub fn take(self) -> (Vec<KeyChord>, Action) {
        (self.key_chords, self.action)
    }

    pub fn mappings_are_same(mapping_1: &Mapping, mapping_2: &Mapping) -> bool {
        if !KeyChord::key_chord_vectors_are_same(
            mapping_1.get_key_chords(),
            mapping_2.get_key_chords(),
        ) {
            return false;
        }

        if mapping_1.get_action() != mapping_2.get_action() {
            return false;
        }

        true
    }
}
