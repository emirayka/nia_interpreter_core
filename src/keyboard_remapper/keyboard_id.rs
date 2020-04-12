#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyboardId {
    id: usize,
}

impl KeyboardId {
    pub fn new(id: usize) -> KeyboardId {
        KeyboardId {
            id,
        }
    }
}