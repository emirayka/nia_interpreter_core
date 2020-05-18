use crate::Action;

#[derive(Clone, Debug)]
pub struct NiaDefineActionCommand {
    action_name: String,
    action: Action,
}

impl NiaDefineActionCommand {
    pub fn new<S>(action_name: S, action: Action) -> NiaDefineActionCommand
    where
        S: Into<String>,
    {
        NiaDefineActionCommand {
            action_name: action_name.into(),
            action,
        }
    }

    pub fn get_action_name(&self) -> &String {
        &self.action_name
    }

    pub fn get_action(&self) -> &Action {
        &self.action
    }
}
