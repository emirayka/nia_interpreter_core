use crate::{Action, NamedAction};

#[derive(Clone, Debug)]
pub struct NiaDefineActionCommand {
    named_action: NamedAction,
}

impl NiaDefineActionCommand {
    pub fn new(named_action: NamedAction) -> NiaDefineActionCommand {
        NiaDefineActionCommand { named_action }
    }

    pub fn get_action(&self) -> &NamedAction {
        &self.named_action
    }

    pub fn take_action(self) -> NamedAction {
        self.named_action
    }
}
