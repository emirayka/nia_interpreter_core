use crate::keyboard_remapper::key_id::KeyId;
use crate::keyboard_remapper::keyboard_id::KeyboardId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyChord {
    modifiers: Vec<(KeyboardId, KeyId)>,
    key: (KeyboardId, KeyId),
}

impl KeyChord {
    pub fn new(modifiers: Vec<(KeyboardId, KeyId)>, key: (KeyboardId, KeyId)) -> KeyChord {
        KeyChord {
            modifiers,
            key,
        }
    }
}