use crate::Error;
use crate::Mapping;

#[derive(Clone, Debug)]
pub enum NiaGetDefinedMappingsCommandResult {
    Success(Vec<Mapping>),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaGetDefinedMappingsCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaGetDefinedMappingsCommandResult::Failure(message)
        } else {
            NiaGetDefinedMappingsCommandResult::Error(message)
        }
    }
}

impl From<Result<Vec<Mapping>, Error>> for NiaGetDefinedMappingsCommandResult {
    fn from(result: Result<Vec<Mapping>, Error>) -> Self {
        match result {
            Ok(mappings) => {
                NiaGetDefinedMappingsCommandResult::Success(mappings)
            }
            Err(error) => NiaGetDefinedMappingsCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaGetDefinedMappingsCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaGetDefinedMappingsCommandResult::Success(defined_mappings) => {
                write!(f, "Success: {:?}.", defined_mappings)
            }
            NiaGetDefinedMappingsCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaGetDefinedMappingsCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
