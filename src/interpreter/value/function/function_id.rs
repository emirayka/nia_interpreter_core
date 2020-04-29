use std::fmt;

use crate::interpreter::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId {
    id: usize,
}

impl FunctionId {
    pub fn new(id: usize) -> FunctionId {
        FunctionId {
            id
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn to_value(&self) -> Value {
        Value::Function(*self)
    }
}

impl fmt::Display for FunctionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
