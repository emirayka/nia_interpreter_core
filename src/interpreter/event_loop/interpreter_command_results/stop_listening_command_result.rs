use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaStopListeningCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaStopListeningCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaStopListeningCommandResult::Failure(message)
        } else {
            NiaStopListeningCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaStopListeningCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaStopListeningCommandResult::Success(),
            Err(error) => NiaStopListeningCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaStopListeningCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaStopListeningCommandResult::Success() => write!(f, "Success."),
            NiaStopListeningCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaStopListeningCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
