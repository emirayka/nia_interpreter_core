#[derive(Clone, Debug)]
pub struct NiaRemoveDeviceByIdCommand {
    device_id: i32,
}

impl NiaRemoveDeviceByIdCommand {
    pub fn new(device_id: i32) -> NiaRemoveDeviceByIdCommand {
        NiaRemoveDeviceByIdCommand { device_id }
    }

    pub fn get_device_id(&self) -> i32 {
        self.device_id
    }
}
