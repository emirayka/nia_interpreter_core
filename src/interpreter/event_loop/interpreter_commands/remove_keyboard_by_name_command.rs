#[derive(Clone, Debug)]
pub struct NiaRemoveKeyboardByNameCommand {
    keyboard_name: String,
}

impl NiaRemoveKeyboardByNameCommand {
    pub fn new<S>(keyboard_name: S) -> NiaRemoveKeyboardByNameCommand
    where
        S: Into<String>,
    {
        NiaRemoveKeyboardByNameCommand {
            keyboard_name: keyboard_name.into(),
        }
    }

    pub fn get_keyboard_name(&self) -> &String {
        &self.keyboard_name
    }
}
