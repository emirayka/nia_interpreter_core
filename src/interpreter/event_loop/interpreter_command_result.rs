#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NiaInterpreterExecutionCommandResult {
    Success(String),
    Error(String),
    Failure(String),
}

impl std::fmt::Display for NiaInterpreterExecutionCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaInterpreterExecutionCommandResult::Success(success_result) => {
                write!(f, "Success: ")?;
                write!(f, "{}", success_result)
            }
            NiaInterpreterExecutionCommandResult::Error(error_result) => {
                write!(f, "Error: ")?;
                write!(f, "{}", error_result)
            }
            NiaInterpreterExecutionCommandResult::Failure(failure_result) => {
                write!(f, "Failure: ")?;
                write!(f, "{}", failure_result)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum NiaInterpreterCommandResult {
    ExecutionResult(NiaInterpreterExecutionCommandResult),
}

impl std::fmt::Display for NiaInterpreterCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NiaInterpreterCommandResult::ExecutionResult(execution_result) => {
                write!(f, "{}", execution_result)
            }
        }
    }
}
