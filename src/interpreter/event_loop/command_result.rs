#[derive(Clone, Debug)]
pub enum ExecutionResult {
    Success(String),
    Error(String),
    Failure(String),
}

impl std::fmt::Display for ExecutionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionResult::Success(success_result) => {
                write!(f, "Success: ");
                write!(f, "{}", success_result)
            },
            ExecutionResult::Error(error_result) => {
                write!(f, "Error: ");
                write!(f, "{}", error_result)
            }
            ExecutionResult::Failure(failure_result) => {
                write!(f, "Failure: ");
                write!(f, "{}", failure_result)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum CommandResult {
    ExecutionResult(ExecutionResult)
}

impl std::fmt::Display for CommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandResult::ExecutionResult(execution_result) => {
                write!(f, "{}", execution_result)
            }
        }
    }
}
