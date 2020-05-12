#[derive(Clone, Debug)]
pub struct NiaRemoveModifierCommand {
    keyboard_path: String,
    key_code: i32,
}

impl NiaRemoveModifierCommand {
    pub fn new<S>(keyboard_path: S, key_code: i32) -> NiaRemoveModifierCommand
    where
        S: Into<String>,
    {
        NiaRemoveModifierCommand {
            keyboard_path: keyboard_path.into(),
            key_code,
        }
    }

    pub fn get_keyboard_path(&self) -> &String {
        &self.keyboard_path
    }

    pub fn get_key_code(&self) -> i32 {
        self.key_code
    }
}
