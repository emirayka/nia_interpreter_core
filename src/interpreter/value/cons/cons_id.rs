use crate::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConsId {
    id: usize
}

impl ConsId {
    pub fn new(id: usize) -> ConsId {
        ConsId {
            id
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn to_value(self) -> Value {
        Value::Cons(self)
    }
}
