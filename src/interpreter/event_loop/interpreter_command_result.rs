use crate::Error;

#[derive(Clone, Debug)]
pub enum NiaInterpreterCommandResult {
    Success(String),
    Error(String),
    Failure(String),
}

impl NiaInterpreterCommandResult {
    pub fn make_success(string: String) -> NiaInterpreterCommandResult {
        NiaInterpreterCommandResult::Success(string)
    }
}

impl From<Error> for NiaInterpreterCommandResult {
    fn from(error: Error) -> Self {
        let message = error.to_string();

        if error.is_failure() {
            NiaInterpreterCommandResult::Failure(message)
        } else {
            NiaInterpreterCommandResult::Error(message)
        }
    }
}

impl<T> From<Result<T, Error>> for NiaInterpreterCommandResult
where
    T: Into<String>,
{
    fn from(result: Result<T, Error>) -> Self {
        match result {
            Ok(t) => NiaInterpreterCommandResult::Success(t.into()),
            Err(error) => NiaInterpreterCommandResult::from(error),
        }
    }
}

impl std::fmt::Display for NiaInterpreterCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaInterpreterCommandResult::Success(execution_result) => {
                write!(f, "Success: {}", execution_result)
            }
            NiaInterpreterCommandResult::Error(execution_result) => {
                write!(f, "Error: {}", execution_result)
            }
            NiaInterpreterCommandResult::Failure(execution_result) => {
                write!(f, "Failure: {}", execution_result)
            }
        }
    }
}
