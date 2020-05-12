#[derive(Clone, Debug)]
pub struct NiaDefineModifierCommand {
    keyboard_path: String,
    key_code: i32,
    modifier_alias: String,
}

impl NiaDefineModifierCommand {
    pub fn new<S>(
        keyboard_path: S,
        key_code: i32,
        modifier_alias: S,
    ) -> NiaDefineModifierCommand
    where
        S: Into<String>,
    {
        NiaDefineModifierCommand {
            keyboard_path: keyboard_path.into(),
            key_code,
            modifier_alias: modifier_alias.into(),
        }
    }

    pub fn get_keyboard_path(&self) -> &String {
        &self.keyboard_path
    }

    pub fn get_key_code(&self) -> i32 {
        self.key_code
    }

    pub fn get_modifier_alias(&self) -> &String {
        &self.modifier_alias
    }
}
