use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaRemoveMappingCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaRemoveMappingCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaRemoveMappingCommandResult::Failure(message)
        } else {
            NiaRemoveMappingCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaRemoveMappingCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaRemoveMappingCommandResult::Success(),
            Err(error) => NiaRemoveMappingCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaRemoveMappingCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaRemoveMappingCommandResult::Success() => write!(f, "Success."),
            NiaRemoveMappingCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaRemoveMappingCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
