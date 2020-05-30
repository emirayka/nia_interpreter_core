use crate::Action;
use crate::KeyChord;

#[derive(Clone, Debug)]
pub struct NiaChangeMappingCommand {
    key_chords: Vec<KeyChord>,
    action: Action,
}

impl NiaChangeMappingCommand {
    pub fn new(
        key_chords: Vec<KeyChord>,
        action: Action,
    ) -> NiaChangeMappingCommand {
        NiaChangeMappingCommand { key_chords, action }
    }

    pub fn get_key_chords(&self) -> &Vec<KeyChord> {
        &self.key_chords
    }

    pub fn get_action(&self) -> &Action {
        &self.action
    }
}
