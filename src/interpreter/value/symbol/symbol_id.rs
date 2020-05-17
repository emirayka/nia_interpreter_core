use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId {
    id: usize,
}

impl SymbolId {
    pub fn new(id: usize) -> SymbolId {
        SymbolId { id }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl fmt::Display for SymbolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
