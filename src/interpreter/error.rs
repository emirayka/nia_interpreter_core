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
    caused_by: Option<Error>,
}

impl Error {
    pub fn from(caused_by: Option<Error>, kind: ErrorKind, message: String) -> Error {
        Error {
            kind,
            message,
            caused_by,
        }
    }

    pub fn empty() -> Error {
        Error {
            kind: ErrorKind::Empty,
            message: String::from(""),
            caused_by: None,
        }
    }
}
