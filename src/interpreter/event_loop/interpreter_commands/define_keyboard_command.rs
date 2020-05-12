#[derive(Clone, Debug)]
pub struct NiaDefineKeyboardCommand {
    keyboard_path: String,
    keyboard_name: String,
}

impl NiaDefineKeyboardCommand {
    pub fn new<S>(
        keyboard_path: S,
        keyboard_name: S,
    ) -> NiaDefineKeyboardCommand
    where
        S: Into<String>,
    {
        NiaDefineKeyboardCommand {
            keyboard_path: keyboard_path.into(),
            keyboard_name: keyboard_name.into(),
        }
    }

    pub fn get_keyboard_path(&self) -> &String {
        &self.keyboard_path
    }

    pub fn get_keyboard_name(&self) -> &String {
        &self.keyboard_name
    }
}
