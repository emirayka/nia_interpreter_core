use crate::Action;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedAction {
    action: Action,
    action_name: String,
}

impl NamedAction {
    pub fn new<S>(action: Action, action_name: S) -> NamedAction
    where
        S: Into<String>,
    {
        NamedAction {
            action,
            action_name: action_name.into(),
        }
    }

    pub fn get_action(&self) -> &Action {
        &self.action
    }

    pub fn take_action(self) -> Action {
        self.action
    }

    pub fn get_action_name(&self) -> &String {
        &self.action_name
    }
}
