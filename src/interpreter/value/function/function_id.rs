use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId {
    id: usize,
}

impl FunctionId {
    pub fn new(id: usize) -> FunctionId {
        FunctionId { id }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl fmt::Display for FunctionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
