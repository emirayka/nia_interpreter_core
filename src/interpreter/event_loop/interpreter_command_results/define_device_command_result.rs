use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaDefineDeviceCommandResult {
    Success(),
    Error(String),
    Failure(String),
}

impl From<Error> for NiaDefineDeviceCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaDefineDeviceCommandResult::Failure(message)
        } else {
            NiaDefineDeviceCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaDefineDeviceCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(_) => NiaDefineDeviceCommandResult::Success(),
            Err(error) => NiaDefineDeviceCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaDefineDeviceCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaDefineDeviceCommandResult::Success() => write!(f, "Success."),
            NiaDefineDeviceCommandResult::Error(execution_result) => {
                write!(f, "Error: {}.", execution_result)
            }
            NiaDefineDeviceCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}.", execution_result)
            }
        }
    }
}
