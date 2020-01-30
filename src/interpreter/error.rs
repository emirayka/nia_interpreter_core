#[derive(Clone, Copy, Debug)]
pub enum EnvironmentErrorKind {
    VariableNotFound,
    FunctionNotFound,
    VariableAlreadyDefined,
    FunctionAlreadyDefined,
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    Environment(EnvironmentErrorKind),
    Empty
}

#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn from(kind: ErrorKind, message: String) -> Error {
        Error {
            kind,
            message,
        }
    }

    pub fn empty() -> Error {
        Error {
            kind: ErrorKind::Empty,
            message: String::from(""),
        }
    }
}
