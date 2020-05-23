use crate::Key;

#[derive(Clone, Debug)]
pub struct NiaRemoveModifierCommand {
    key: Key,
}

impl NiaRemoveModifierCommand {
    pub fn new(key: Key) -> NiaRemoveModifierCommand {
        NiaRemoveModifierCommand { key }
    }

    pub fn get_key(&self) -> Key {
        self.key
    }
}
