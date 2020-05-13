use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaRemoveDeviceByNameCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaRemoveDeviceByNameCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaRemoveDeviceByNameCommandResult::Failure(message)
        } else {
            NiaRemoveDeviceByNameCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaRemoveDeviceByNameCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaRemoveDeviceByNameCommandResult::Success(),
            Err(error) => NiaRemoveDeviceByNameCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaRemoveDeviceByNameCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaRemoveDeviceByNameCommandResult::Success() => {
                write!(f, "Success.")
            }
            NiaRemoveDeviceByNameCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaRemoveDeviceByNameCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
