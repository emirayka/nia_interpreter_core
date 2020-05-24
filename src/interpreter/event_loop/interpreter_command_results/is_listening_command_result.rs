use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaIsListeningCommandResult {
    Success(bool),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaIsListeningCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaIsListeningCommandResult::Failure(message)
        } else {
            NiaIsListeningCommandResult::Error(message)
        }
    }
}

impl From<Result<bool, Error>> for NiaIsListeningCommandResult {
    fn from(result: Result<bool, Error>) -> Self {
        match result {
            Ok(is_listening) => {
                NiaIsListeningCommandResult::Success(is_listening)
            }
            Err(error) => NiaIsListeningCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaIsListeningCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaIsListeningCommandResult::Success(execution_result) => {
                write!(f, "Success: {}.", execution_result)
            }
            NiaIsListeningCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaIsListeningCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
