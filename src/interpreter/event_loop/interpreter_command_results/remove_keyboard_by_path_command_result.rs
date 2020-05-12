use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaRemoveKeyboardByPathCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaRemoveKeyboardByPathCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaRemoveKeyboardByPathCommandResult::Failure(message)
        } else {
            NiaRemoveKeyboardByPathCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaRemoveKeyboardByPathCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaRemoveKeyboardByPathCommandResult::Success(),
            Err(error) => NiaRemoveKeyboardByPathCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaRemoveKeyboardByPathCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaRemoveKeyboardByPathCommandResult::Success() => {
                write!(f, "Success.")
            }
            NiaRemoveKeyboardByPathCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaRemoveKeyboardByPathCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
