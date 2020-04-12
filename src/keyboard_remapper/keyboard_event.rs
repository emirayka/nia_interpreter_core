use crate::keyboard_remapper::key_id::KeyId;
use crate::keyboard_remapper::keyboard_id::KeyboardId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyboardEventType {
    RELEASED,
    PRESSED,
    UNKNOWN,
}

impl KeyboardEventType {
    pub fn from_value(value: i32) -> KeyboardEventType {
        if value == 0 {
            KeyboardEventType::RELEASED
        } else if value == 1 {
            KeyboardEventType::PRESSED
        } else {
            KeyboardEventType::UNKNOWN
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyboardEvent {
    keyboard_id: KeyboardId,
    key_id: KeyId,
    event_type: KeyboardEventType,
}

impl KeyboardEvent {
    pub fn new(keyboard_id: KeyboardId, key_id: KeyId, event_type: KeyboardEventType) -> KeyboardEvent {
        KeyboardEvent {
            keyboard_id,
            key_id,
            event_type,
        }
    }

    pub fn get_keyboard_id(&self) -> KeyboardId {
        self.keyboard_id
    }

    pub fn get_key_id(&self) -> KeyId {
        self.key_id
    }

    pub fn get_event_type(&self) -> KeyboardEventType {
        self.event_type
    }
}