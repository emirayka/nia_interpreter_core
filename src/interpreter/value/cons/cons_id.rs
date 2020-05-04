use crate::Value;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConsId {
    id: usize,
}

impl ConsId {
    pub fn new(id: usize) -> ConsId {
        ConsId { id }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl fmt::Display for ConsId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
