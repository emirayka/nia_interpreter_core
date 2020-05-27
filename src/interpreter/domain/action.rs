use crate::ActionKeyCategory;
use crate::NamedAction;
use crate::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    KeyPress(ActionKeyCategory, i32),
    KeyClick(ActionKeyCategory, i32),
    KeyRelease(ActionKeyCategory, i32),

    MouseButtonPress(i32),
    MouseButtonClick(i32),
    MouseButtonRelease(i32),

    ActionTextKeyClick(i32),
    ActionNumberKeyClick(i32),
    ActionFunctionKeyClick(i32),
    ActionControlKeyClick(i32),
    ActionKPKeyClick(i32),
    ActionMultimediaKeyClick(i32),
    ActionMouseButtonKeyClick(i32),

    MouseAbsoluteMove(i32, i32),
    MouseRelativeMove(i32, i32),

    TextType(String),
    Wait(i32),

    ExecuteCode(String),
    ExecuteFunction(String),
    ExecuteOSCommand(String),
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
