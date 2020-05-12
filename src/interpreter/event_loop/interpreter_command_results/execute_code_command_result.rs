use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaExecuteCodeCommandResult {
    Success(String),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaExecuteCodeCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaExecuteCodeCommandResult::Failure(message)
        } else {
            NiaExecuteCodeCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaExecuteCodeCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(t) => NiaExecuteCodeCommandResult::Success(t.into()),
            Err(error) => NiaExecuteCodeCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaExecuteCodeCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaExecuteCodeCommandResult::Success(execution_result) => {
                write!(f, "Success: {}.", execution_result)
            }
            NiaExecuteCodeCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaExecuteCodeCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
