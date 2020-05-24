use crate::Action;
use crate::Error;
use crate::NamedAction;

#[derive(Clone, Debug)]
pub enum NiaGetDefinedActionsCommandResult {
    Success(Vec<NamedAction>),
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

impl From<Result<Vec<NamedAction>, Error>>
    for NiaGetDefinedActionsCommandResult
{
    fn from(result: Result<Vec<NamedAction>, Error>) -> Self {
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
