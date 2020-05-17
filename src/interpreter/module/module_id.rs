use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleId {
    id: usize,
}

impl ModuleId {
    pub fn new(id: usize) -> ModuleId {
        ModuleId { id }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl fmt::Display for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<ModuleId: {}>", self.id)
    }
}
