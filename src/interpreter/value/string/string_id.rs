use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringId {
    id: usize
}

impl StringId {
    pub fn new(id: usize) -> StringId {
        StringId {
            id
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl fmt::Display for StringId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
