use crate::interpreter::Value;

#[derive(Clone, Debug)]
pub enum Action {
    Empty,
    Execute(Value), // value must be a function that has no params
}

impl From<Value> for Action {
    fn from(value: Value) -> Self {
        Action::Execute(value)
    }
}