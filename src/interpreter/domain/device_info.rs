#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceInfo {
    id: i32,
    path: String,
    name: String,
}

impl DeviceInfo {
    pub fn new<S>(id: i32, path: S, name: S) -> Self
    where
        S: Into<String>,
    {
        DeviceInfo {
            id,
            path: path.into(),
            name: name.into(),
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
