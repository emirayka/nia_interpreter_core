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
    caused_by: Option<Box<Error>>,
}

impl Error {
    pub fn from(caused_by: Option<Error>, kind: ErrorKind, message: String) -> Error {
        Error {
            kind,
            message,
            caused_by: match caused_by {
                Some(error) => Some(Box::new(error)),
                None => None
            },
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
