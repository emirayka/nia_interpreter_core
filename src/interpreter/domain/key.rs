use crate::Convertable;
use crate::DeviceKey;
use crate::LoneKey;

#[derive(Debug, Clone, Copy, Eq)]
pub enum Key {
    DeviceKey(DeviceKey),
    LoneKey(LoneKey),
}

impl Key {
    pub fn new_device_key(device_id: i32, key_id: i32) -> Key {
        Key::DeviceKey(DeviceKey::new(device_id, key_id))
    }

    pub fn new_lone_key(key_id: i32) -> Key {
        Key::LoneKey(LoneKey::new(key_id))
    }

    pub fn get_device_id(&self) -> Option<i32> {
        match self {
            Key::DeviceKey(device_key) => Some(device_key.get_device_id()),
            Key::LoneKey(_) => None,
        }
    }

    pub fn get_key_id(&self) -> i32 {
        match self {
            Key::DeviceKey(device_key) => device_key.get_key_id(),
            Key::LoneKey(lone_key) => lone_key.get_key_id(),
        }
    }

    pub fn keys_are_same(key1: Key, key2: Key) -> bool {
        match (key1, key2) {
            (Key::DeviceKey(device_key_1), Key::DeviceKey(device_key_2)) => {
                device_key_1.get_device_id() == device_key_2.get_device_id()
                    && device_key_1.get_key_id() == device_key_2.get_key_id()
            }
            (Key::LoneKey(lone_key_1), Key::LoneKey(lone_key_2)) => {
                lone_key_1.get_key_id() == lone_key_2.get_key_id()
            }
            _ => false,
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        use Key::*;

        match (self, other) {
            (LoneKey(lone_key_1), LoneKey(lone_key_2)) => {
                lone_key_1 == lone_key_2
            }
            (LoneKey(lone_key_1), DeviceKey(device_key_2)) => {
                lone_key_1.get_key_id() == device_key_2.get_key_id()
            }
            (DeviceKey(device_key_1), LoneKey(lone_key_2)) => {
                device_key_1.get_key_id() == lone_key_2.get_key_id()
            }
            (DeviceKey(device_key_1), DeviceKey(device_key_2)) => {
                device_key_1 == device_key_2
            }
        }
    }
}

impl Convertable<Key, nia_events::Key> for Key {
    fn to_nia_events_representation(&self) -> nia_events::Key {
        match self {
            Key::DeviceKey(device_key) => nia_events::Key::Key2(
                nia_events::DeviceId::new(device_key.get_device_id() as u16),
                nia_events::KeyId::new(device_key.get_key_id() as u16),
            ),
            Key::LoneKey(lone_key) => nia_events::Key::Key1(
                nia_events::KeyId::new(lone_key.get_key_id() as u16),
            ),
        }
    }

    fn from_nia_events_representation(value: &nia_events::Key) -> Key {
        match value {
            nia_events::Key::Key1(key_id) => {
                Key::LoneKey(LoneKey::new(key_id.get_id() as i32))
            }
            nia_events::Key::Key2(device_id, key_id) => {
                Key::DeviceKey(DeviceKey::new(
                    device_id.get_id() as i32,
                    key_id.get_id() as i32,
                ))
            }
        }
    }
}

#[macro_export]
macro_rules! nia_key {
    ($key_id:expr) => {
        $crate::Key::LoneKey($crate::LoneKey::new($key_id));
    };
    ($device_id:expr, $key_id:expr) => {
        $crate::Key::DeviceKey($crate::DeviceKey::new($device_id, $key_id));
    };
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[cfg(test)]
    mod eq {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn equality_is_correct() {
            let specs = vec![
                (nia_key!(1), nia_key!(1), true),
                (nia_key!(1), nia_key!(2), false),
                (nia_key!(2), nia_key!(1), false),
                (nia_key!(2), nia_key!(2), true),
                (nia_key!(1, 1), nia_key!(1), true),
                (nia_key!(1, 1), nia_key!(2), false),
                (nia_key!(1, 2), nia_key!(1), false),
                (nia_key!(1, 2), nia_key!(2), true),
                (nia_key!(2, 1), nia_key!(1), true),
                (nia_key!(2, 1), nia_key!(2), false),
                (nia_key!(2, 2), nia_key!(1), false),
                (nia_key!(2, 2), nia_key!(2), true),
                (nia_key!(1), nia_key!(1, 1), true),
                (nia_key!(1), nia_key!(1, 2), false),
                (nia_key!(1), nia_key!(2, 1), true),
                (nia_key!(1), nia_key!(2, 2), false),
                (nia_key!(2), nia_key!(1, 1), false),
                (nia_key!(2), nia_key!(1, 2), true),
                (nia_key!(2), nia_key!(2, 1), false),
                (nia_key!(2), nia_key!(2, 2), true),
                (nia_key!(1, 1), nia_key!(1, 1), true),
                (nia_key!(1, 1), nia_key!(1, 2), false),
                (nia_key!(1, 1), nia_key!(2, 1), false),
                (nia_key!(1, 1), nia_key!(2, 2), false),
            ];

            for (key_1, key_2, expected) in specs {
                nia_assert_equal(expected, key_1 == key_2);
            }
        }
    }

    #[cfg(test)]
    mod keys_are_same {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn returns_correct_result() {
            let specs = vec![
                (nia_key!(1), nia_key!(1), true),
                (nia_key!(1), nia_key!(2), false),
                (nia_key!(2), nia_key!(1), false),
                (nia_key!(2), nia_key!(2), true),
                (nia_key!(1, 1), nia_key!(1), false),
                (nia_key!(1, 1), nia_key!(2), false),
                (nia_key!(1, 2), nia_key!(1), false),
                (nia_key!(1, 2), nia_key!(2), false),
                (nia_key!(2, 1), nia_key!(1), false),
                (nia_key!(2, 1), nia_key!(2), false),
                (nia_key!(2, 2), nia_key!(1), false),
                (nia_key!(2, 2), nia_key!(2), false),
                (nia_key!(1), nia_key!(1, 1), false),
                (nia_key!(1), nia_key!(1, 2), false),
                (nia_key!(1), nia_key!(2, 1), false),
                (nia_key!(1), nia_key!(2, 2), false),
                (nia_key!(2), nia_key!(1, 1), false),
                (nia_key!(2), nia_key!(1, 2), false),
                (nia_key!(2), nia_key!(2, 1), false),
                (nia_key!(2), nia_key!(2, 2), false),
                (nia_key!(1, 1), nia_key!(1, 1), true),
                (nia_key!(1, 1), nia_key!(1, 2), false),
                (nia_key!(1, 1), nia_key!(2, 1), false),
                (nia_key!(1, 1), nia_key!(2, 2), false),
            ];

            for (key_1, key_2, expected) in specs {
                nia_assert_equal(expected, Key::keys_are_same(key_1, key_2));
            }
        }
    }

    #[cfg(test)]
    mod convertable_between_interpreter_and_nia_events_representations {
        #[allow(unused_imports)]
        use super::*;
    }
}
