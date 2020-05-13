#[derive(Clone, Debug)]
pub struct NiaDefineDeviceCommand {
    device_path: String,
    device_name: String,
}

impl NiaDefineDeviceCommand {
    pub fn new<S>(device_path: S, device_name: S) -> NiaDefineDeviceCommand
    where
        S: Into<String>,
    {
        NiaDefineDeviceCommand {
            device_path: device_path.into(),
            device_name: device_name.into(),
        }
    }

    pub fn get_device_path(&self) -> &String {
        &self.device_path
    }

    pub fn get_device_name(&self) -> &String {
        &self.device_name
    }
}
