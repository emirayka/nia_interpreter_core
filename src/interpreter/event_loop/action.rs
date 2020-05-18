#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    KeyPress(i32),
    KeyClick(i32),
    KeyRelease(i32),

    MouseButtonPress(i32),
    MouseButtonClick(i32),
    MouseButtonRelease(i32),

    MouseAbsoluteMove(i32, i32),
    MouseRelativeMove(i32, i32),

    TextType(String),
    Wait(i32),

    ExecuteCode(String),
    ExecuteFunction(String),
    ExecuteOSCommand(String),
}
