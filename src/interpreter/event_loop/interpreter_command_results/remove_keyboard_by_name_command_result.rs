use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaRemoveKeyboardByNameCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaRemoveKeyboardByNameCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaRemoveKeyboardByNameCommandResult::Failure(message)
        } else {
            NiaRemoveKeyboardByNameCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaRemoveKeyboardByNameCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaRemoveKeyboardByNameCommandResult::Success(),
            Err(error) => NiaRemoveKeyboardByNameCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaRemoveKeyboardByNameCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaRemoveKeyboardByNameCommandResult::Success() => {
                write!(f, "Success.")
            }
            NiaRemoveKeyboardByNameCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaRemoveKeyboardByNameCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
