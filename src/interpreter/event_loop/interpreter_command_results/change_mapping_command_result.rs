use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaChangeMappingCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaChangeMappingCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaChangeMappingCommandResult::Failure(message)
        } else {
            NiaChangeMappingCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaChangeMappingCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaChangeMappingCommandResult::Success(),
            Err(error) => NiaChangeMappingCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaChangeMappingCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaChangeMappingCommandResult::Success() => write!(f, "Success."),
            NiaChangeMappingCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaChangeMappingCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
