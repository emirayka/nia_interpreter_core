use evdev_rs::enums::EV_KEY;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyId {
    id: u16,
}

impl KeyId {
    pub fn new(id: u16) -> KeyId {
        KeyId {
            id,
        }
    }

    pub fn from_ev_key(ev_key: EV_KEY) -> KeyId {
        KeyId {
            id: ev_key as u16,
        }
    }
}