#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceKey {
    device_id: i32,
    key_id: i32,
}

impl DeviceKey {
    pub fn new(device_id: i32, key_id: i32) -> DeviceKey {
        DeviceKey { key_id, device_id }
    }

    pub fn get_key_id(&self) -> i32 {
        self.key_id
    }

    pub fn get_device_id(&self) -> i32 {
        self.device_id
    }

    pub fn set_key_id(&mut self, key_id: i32) {
        self.key_id = key_id
    }

    pub fn set_device_id(&mut self, device_id: i32) {
        self.device_id = device_id
    }
}
