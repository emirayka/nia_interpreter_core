#[derive(Clone, Debug)]
pub struct NiaDefineModifierCommand {
    device_id: i32,
    key_code: i32,
    modifier_alias: String,
}

impl NiaDefineModifierCommand {
    pub fn new<S>(
        device_id: i32,
        key_code: i32,
        modifier_alias: S,
    ) -> NiaDefineModifierCommand
    where
        S: Into<String>,
    {
        NiaDefineModifierCommand {
            device_id,
            key_code,
            modifier_alias: modifier_alias.into(),
        }
    }

    pub fn get_device_id(&self) -> i32 {
        self.device_id
    }

    pub fn get_key_code(&self) -> i32 {
        self.key_code
    }

    pub fn get_modifier_alias(&self) -> &String {
        &self.modifier_alias
    }
}
