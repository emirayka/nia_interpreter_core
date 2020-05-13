#[derive(Clone, Debug)]
pub struct NiaRemoveModifierCommand {
    device_id: i32,
    key_code: i32,
}

impl NiaRemoveModifierCommand {
    pub fn new(device_id: i32, key_code: i32) -> NiaRemoveModifierCommand {
        NiaRemoveModifierCommand {
            device_id,
            key_code,
        }
    }

    pub fn get_device_id(&self) -> i32 {
        self.device_id
    }

    pub fn get_key_code(&self) -> i32 {
        self.key_code
    }
}
