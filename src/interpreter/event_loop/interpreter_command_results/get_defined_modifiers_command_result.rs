use crate::{Error, ModifierDescription};

#[derive(Clone, Debug)]
pub enum NiaGetDefinedModifiersCommandResult {
    Success(Vec<ModifierDescription>),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaGetDefinedModifiersCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaGetDefinedModifiersCommandResult::Failure(message)
        } else {
            NiaGetDefinedModifiersCommandResult::Error(message)
        }
    }
}

impl From<Result<Vec<ModifierDescription>, Error>>
    for NiaGetDefinedModifiersCommandResult
{
    fn from(result: Result<Vec<ModifierDescription>, Error>) -> Self {
        match result {
            Ok(modifiers) => {
                NiaGetDefinedModifiersCommandResult::Success(modifiers)
            }
            Err(error) => NiaGetDefinedModifiersCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaGetDefinedModifiersCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaGetDefinedModifiersCommandResult::Success(defined_modifiers) => {
                write!(f, "Success: {:?}.", defined_modifiers)
            }
            NiaGetDefinedModifiersCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaGetDefinedModifiersCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
