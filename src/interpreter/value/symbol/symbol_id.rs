use crate::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId {
    id: usize
}

impl SymbolId {
    pub fn new(id: usize) -> SymbolId {
        SymbolId {
            id
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn to_value(&self) -> Value {
        Value::Symbol(*self)
    }
}
