use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaDefineKeyboardCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaDefineKeyboardCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaDefineKeyboardCommandResult::Failure(message)
        } else {
            NiaDefineKeyboardCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaDefineKeyboardCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaDefineKeyboardCommandResult::Success(),
            Err(error) => NiaDefineKeyboardCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaDefineKeyboardCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaDefineKeyboardCommandResult::Success() => write!(f, "Success."),
            NiaDefineKeyboardCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaDefineKeyboardCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
