use crate::KeyChord;

#[derive(Clone, Debug)]
pub struct NiaRemoveMappingCommand {
    key_chord_vector: Vec<KeyChord>,
}

impl NiaRemoveMappingCommand {
    pub fn new(key_chord_vector: Vec<KeyChord>) -> NiaRemoveMappingCommand {
        NiaRemoveMappingCommand { key_chord_vector }
    }

    pub fn get_key_chords(&self) -> &Vec<KeyChord> {
        &self.key_chord_vector
    }
}
