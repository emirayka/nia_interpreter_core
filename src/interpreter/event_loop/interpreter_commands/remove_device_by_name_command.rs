#[derive(Clone, Debug)]
pub struct NiaRemoveDeviceByNameCommand {
    device_name: String,
}

impl NiaRemoveDeviceByNameCommand {
    pub fn new<S>(keyboard_name: S) -> NiaRemoveDeviceByNameCommand
    where
        S: Into<String>,
    {
        NiaRemoveDeviceByNameCommand {
            device_name: keyboard_name.into(),
        }
    }

    pub fn get_device_name(&self) -> &String {
        &self.device_name
    }
}
