use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaDefineMappingCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaDefineMappingCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaDefineMappingCommandResult::Failure(message)
        } else {
            NiaDefineMappingCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaDefineMappingCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaDefineMappingCommandResult::Success(),
            Err(error) => NiaDefineMappingCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaDefineMappingCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaDefineMappingCommandResult::Success() => write!(f, "Success."),
            NiaDefineMappingCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaDefineMappingCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
