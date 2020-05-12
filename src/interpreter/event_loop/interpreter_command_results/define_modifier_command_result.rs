use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaDefineModifierCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaDefineModifierCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaDefineModifierCommandResult::Failure(message)
        } else {
            NiaDefineModifierCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaDefineModifierCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaDefineModifierCommandResult::Success(),
            Err(error) => NiaDefineModifierCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaDefineModifierCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaDefineModifierCommandResult::Success() => write!(f, "Success."),
            NiaDefineModifierCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaDefineModifierCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
