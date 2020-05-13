#[derive(Clone, Debug)]
pub struct NiaRemoveDeviceByPathCommand {
    device_path: String,
}

impl NiaRemoveDeviceByPathCommand {
    pub fn new<S>(keyboard_path: S) -> NiaRemoveDeviceByPathCommand
    where
        S: Into<String>,
    {
        NiaRemoveDeviceByPathCommand {
            device_path: keyboard_path.into(),
        }
    }

    pub fn get_device_path(&self) -> &String {
        &self.device_path
    }
}
