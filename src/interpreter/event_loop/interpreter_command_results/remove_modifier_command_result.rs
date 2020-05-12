use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaRemoveModifierCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaRemoveModifierCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaRemoveModifierCommandResult::Failure(message)
        } else {
            NiaRemoveModifierCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaRemoveModifierCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaRemoveModifierCommandResult::Success(),
            Err(error) => NiaRemoveModifierCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaRemoveModifierCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaRemoveModifierCommandResult::Success() => write!(f, "Success."),
            NiaRemoveModifierCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaRemoveModifierCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
