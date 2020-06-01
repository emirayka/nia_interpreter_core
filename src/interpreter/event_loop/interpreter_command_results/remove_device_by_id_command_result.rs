use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaRemoveDeviceByIdCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaRemoveDeviceByIdCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaRemoveDeviceByIdCommandResult::Failure(message)
        } else {
            NiaRemoveDeviceByIdCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaRemoveDeviceByIdCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaRemoveDeviceByIdCommandResult::Success(),
            Err(error) => NiaRemoveDeviceByIdCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaRemoveDeviceByIdCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaRemoveDeviceByIdCommandResult::Success() => {
                write!(f, "Success.")
            }
            NiaRemoveDeviceByIdCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaRemoveDeviceByIdCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
