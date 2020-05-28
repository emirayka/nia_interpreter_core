use crate::NamedAction;
use crate::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    KeyPress(i32),
    KeyClick(i32),
    KeyRelease(i32),

    MouseButtonPress(i32),
    MouseButtonClick(i32),
    MouseButtonRelease(i32),

    TextKeyClick(i32),
    NumberKeyClick(i32),
    FunctionKeyClick(i32),
    ControlKeyClick(i32),
    KPKeyClick(i32),
    MultimediaKeyClick(i32),
    MouseButtonKeyClick(i32),

    MouseAbsoluteMove(i32, i32),
    MouseRelativeMove(i32, i32),

    TextType(String),
    ExecuteCode(String),
    ExecuteFunction(String),
    ExecuteOSCommand(String),
    ExecuteNamedAction(String),
    Wait(i32),

    ExecuteFunctionValue(Value), // specified only for interpreter
}

impl Action {
    pub fn into_named<S>(self, name: S) -> NamedAction
    where
        S: Into<String>,
    {
        NamedAction::new(self, name.into())
    }
}
