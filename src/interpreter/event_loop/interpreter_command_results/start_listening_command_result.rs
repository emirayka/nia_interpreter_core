use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaStartListeningCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaStartListeningCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaStartListeningCommandResult::Failure(message)
        } else {
            NiaStartListeningCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaStartListeningCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaStartListeningCommandResult::Success(),
            Err(error) => NiaStartListeningCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaStartListeningCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaStartListeningCommandResult::Success() => write!(f, "Success."),
            NiaStartListeningCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaStartListeningCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
