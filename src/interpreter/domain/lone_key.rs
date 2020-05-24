#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoneKey {
    key_id: i32,
}

impl LoneKey {
    pub fn new(key_id: i32) -> LoneKey {
        LoneKey { key_id }
    }

    pub fn get_key_id(&self) -> i32 {
        self.key_id
    }

    pub fn set_key_id(&mut self, key_id: i32) {
        self.key_id = key_id
    }
}
