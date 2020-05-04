#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaString {
    string: String,
}

impl NiaString {
    pub fn new(string: String) -> NiaString {
        NiaString { string }
    }

    pub fn get_string(&self) -> &String {
        &self.string
    }
}
