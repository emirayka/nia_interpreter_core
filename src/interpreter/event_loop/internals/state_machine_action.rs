use crate::interpreter::Value;
use crate::Action;

#[derive(Clone, Debug)]
pub enum StateMachineAction {
    Empty,
    Execute(Action),
}

impl From<Action> for StateMachineAction {
    fn from(action: Action) -> Self {
        StateMachineAction::Execute(action)
    }
}
