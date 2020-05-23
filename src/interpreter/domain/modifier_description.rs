use crate::Key;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModifierDescription {
    key: Key,
    name: String,
}

impl ModifierDescription {
    pub fn new<S>(key: Key, name: S) -> ModifierDescription
    where
        S: Into<String>,
    {
        ModifierDescription {
            key,
            name: name.into(),
        }
    }

    pub fn get_key(&self) -> Key {
        self.key
    }

    pub fn get_alias(&self) -> &String {
        &self.name
    }
}

#[macro_export]
macro_rules! nia_modifier {
    ($key_id:expr, $alias:expr) => {
        $crate::ModifierDescription::new($crate::nia_key!($key_id), $alias);
    };
    ($device_id:expr, $key_id:expr, $alias:expr) => {
        $crate::ModifierDescription::new(
            $crate::nia_key!($device_id, $key_id),
            $alias,
        );
    };
}
