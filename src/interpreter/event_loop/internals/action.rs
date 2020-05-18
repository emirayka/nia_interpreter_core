use crate::interpreter::Value;

#[derive(Clone, Debug)]
pub enum StateMachineAction {
    Empty,
    Execute(Value), // value must be a function that can be invoked with no params
}

impl From<Value> for StateMachineAction {
    fn from(value: Value) -> Self {
        StateMachineAction::Execute(value)
    }
}
