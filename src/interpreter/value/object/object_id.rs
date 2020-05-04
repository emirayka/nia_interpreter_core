use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId {
    id: usize,
}

impl ObjectId {
    pub fn new(index: usize) -> ObjectId {
        ObjectId { id: index }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
