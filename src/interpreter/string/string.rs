#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VString {
    string: String
}

impl VString {
    pub fn new(string: String) -> VString {
        VString {
            string
        }
    }

    pub fn get_string(&self) -> &String {
        &self.string
    }
}