use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaRemoveDeviceByPathCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaRemoveDeviceByPathCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaRemoveDeviceByPathCommandResult::Failure(message)
        } else {
            NiaRemoveDeviceByPathCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaRemoveDeviceByPathCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaRemoveDeviceByPathCommandResult::Success(),
            Err(error) => NiaRemoveDeviceByPathCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaRemoveDeviceByPathCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaRemoveDeviceByPathCommandResult::Success() => {
                write!(f, "Success.")
            }
            NiaRemoveDeviceByPathCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaRemoveDeviceByPathCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
