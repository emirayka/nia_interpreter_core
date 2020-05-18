use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaDefineActionCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaDefineActionCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaDefineActionCommandResult::Failure(message)
        } else {
            NiaDefineActionCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaDefineActionCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaDefineActionCommandResult::Success(),
            Err(error) => NiaDefineActionCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaDefineActionCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaDefineActionCommandResult::Success() => write!(f, "Success."),
            NiaDefineActionCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaDefineActionCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
