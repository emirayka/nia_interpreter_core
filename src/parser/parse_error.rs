use std::fmt;

#[derive(Debug, Clone)]
pub enum ParseError {
    TrailingInput(String),
    NomError((String, nom::error::ErrorKind)),
    NomFailure((String, nom::error::ErrorKind)),
    NomIncomplete(),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::TrailingInput(string) => {
                write!(f, "Trailing input: {}", string)
            },
            ParseError::NomError((string, error_kind)) => {
                write!(f, "Parse error: {} {:?}", string, error_kind)
            },
            ParseError::NomFailure((string, error_kind)) => {
                write!(f, "Parse failure: {} {:?}", string, error_kind)
            },
            ParseError::NomIncomplete() => write!(f, "Incomplete."),
        }
    }
}
