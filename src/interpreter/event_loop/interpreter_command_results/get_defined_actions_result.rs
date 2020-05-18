use crate::{Action, Error};

#[derive(Clone, Debug)]
pub enum NiaGetDefinedActionsCommandResult {
    Success(Vec<(String, Action)>),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaGetDefinedActionsCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaGetDefinedActionsCommandResult::Failure(message)
        } else {
            NiaGetDefinedActionsCommandResult::Error(message)
        }
    }
}

impl From<Result<Vec<(String, Action)>, Error>>
    for NiaGetDefinedActionsCommandResult
{
    fn from(result: Result<Vec<(String, Action)>, Error>) -> Self {
        match result {
            Ok(result) => NiaGetDefinedActionsCommandResult::Success(result),
            Err(error) => NiaGetDefinedActionsCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaGetDefinedActionsCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaGetDefinedActionsCommandResult::Success(result) => {
                write!(f, "{:?}.", result)
            }
            NiaGetDefinedActionsCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaGetDefinedActionsCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
