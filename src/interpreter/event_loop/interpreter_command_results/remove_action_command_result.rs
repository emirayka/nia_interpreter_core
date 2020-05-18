use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaRemoveActionCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaRemoveActionCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaRemoveActionCommandResult::Failure(message)
        } else {
            NiaRemoveActionCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaRemoveActionCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaRemoveActionCommandResult::Success(),
            Err(error) => NiaRemoveActionCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaRemoveActionCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaRemoveActionCommandResult::Success() => write!(f, "Success."),
            NiaRemoveActionCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaRemoveActionCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
