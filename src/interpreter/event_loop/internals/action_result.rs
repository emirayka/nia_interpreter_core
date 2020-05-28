use crate::Action;
use nia_events::Command;

#[derive(Clone, Debug)]
pub enum ActionResult {
    SendCommand(Command),
    PushAction(Action),
    Nothing,
}

impl From<Command> for ActionResult {
    fn from(command: Command) -> Self {
        ActionResult::SendCommand(command)
    }
}

impl From<Action> for ActionResult {
    fn from(action: Action) -> Self {
        ActionResult::PushAction(action)
    }
}
