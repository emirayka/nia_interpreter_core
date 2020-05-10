#[derive(Clone, Debug)]
pub enum NiaInterpreterCommand {
    ExecuteCode(String),
    DefineKeyboard(String, String),
    RemoveKeyboardByPath(String),
    RemoveKeyboardByName(String),
}
