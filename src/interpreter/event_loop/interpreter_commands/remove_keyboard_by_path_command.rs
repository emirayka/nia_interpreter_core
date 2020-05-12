#[derive(Clone, Debug)]
pub struct NiaRemoveKeyboardByPathCommand {
    keyboard_path: String,
}

impl NiaRemoveKeyboardByPathCommand {
    pub fn new<S>(keyboard_path: S) -> NiaRemoveKeyboardByPathCommand
    where
        S: Into<String>,
    {
        NiaRemoveKeyboardByPathCommand {
            keyboard_path: keyboard_path.into(),
        }
    }

    pub fn get_keyboard_path(&self) -> &String {
        &self.keyboard_path
    }
}
