#[derive(Clone, Debug)]
pub struct NiaRemoveActionCommand {
    action_name: String,
}

impl NiaRemoveActionCommand {
    pub fn new<S>(action_name: S) -> NiaRemoveActionCommand
    where
        S: Into<String>,
    {
        NiaRemoveActionCommand {
            action_name: action_name.into(),
        }
    }

    pub fn get_action_name(&self) -> &String {
        &self.action_name
    }
}
