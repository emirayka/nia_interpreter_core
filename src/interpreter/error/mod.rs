pub mod assertion;

use crate::interpreter::symbol::Symbol;
use crate::interpreter::interpreter::Interpreter;

pub const SYMBOL_NAME_INVALID_ARGUMENT: &'static str = "invalid-argument";
pub const SYMBOL_NAME_INVALID_ARGUMENT_COUNT: &'static str = "invalid-argument-count";
pub const SYMBOL_NAME_GENERIC_EXECUTION_ERROR: &'static str = "generic-execution-error";

#[derive(Clone, Copy, Debug)]
pub enum EnvironmentErrorKind {
    VariableNotFound,
    FunctionNotFound,
    VariableAlreadyDefined,
    FunctionAlreadyDefined,
}

#[derive(Clone, Copy, Debug)]
pub enum ExecutionErrorKind {
    Generic,
}

#[derive(Clone, Copy, Debug)]
pub enum ArgumentErrorKind {
    InvalidArgument,
    InvalidArgumentCount,
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    Environment(EnvironmentErrorKind),
    Execution(ExecutionErrorKind),
    Argument(ArgumentErrorKind),
    Empty,
}

#[derive(Clone, Debug)]
pub struct Error {
    error_kind: ErrorKind,
    message: String,
    caused_by: Option<Box<Error>>,
    symbol: Option<Symbol>,
}

impl Error {
    pub fn get_error_kind(&self) -> ErrorKind {
        self.error_kind
    }

    pub fn get_symbol(&self) -> Option<Symbol> {
        self.symbol.clone()
    }

    pub fn get_total_cause(&self) -> &Error {
        match &self.caused_by {
            Some(b) => b.get_total_cause(),
            None => self
        }
    }
}

impl Error {
    pub fn from(caused_by: Option<Error>, kind: ErrorKind, message: &str, symbol: Symbol) -> Error {
        Error {
            error_kind: kind,
            message: String::from(message),
            caused_by: match caused_by {
                Some(error) => Some(Box::new(error)),
                None => None
            },
            symbol: Some(symbol)
        }
    }

    pub fn empty() -> Error {
        Error {
            error_kind: ErrorKind::Empty,
            message: String::from(""),
            caused_by: None,
            symbol: None
        }
    }

    pub fn invalid_argument(interpreter: &mut Interpreter, message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::Argument(ArgumentErrorKind::InvalidArgument),
            message,
            interpreter.intern_symbol(SYMBOL_NAME_INVALID_ARGUMENT)
        )
    }

    pub fn invalid_argument_caused(interpreter: &mut Interpreter, message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::Argument(ArgumentErrorKind::InvalidArgument),
            message,
            interpreter.intern_symbol(SYMBOL_NAME_INVALID_ARGUMENT)
        )
    }

    pub fn invalid_argument_count(interpreter: &mut Interpreter, message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::Argument(ArgumentErrorKind::InvalidArgumentCount),
            message,
            interpreter.intern_symbol(SYMBOL_NAME_INVALID_ARGUMENT_COUNT)
        )
    }

    pub fn invalid_argument_count_caused(interpreter: &mut Interpreter, message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::Argument(ArgumentErrorKind::InvalidArgumentCount),
            message,
            interpreter.intern_symbol(SYMBOL_NAME_INVALID_ARGUMENT_COUNT)
        )
    }

    pub fn generic_execution_error_caused(interpreter: &mut Interpreter, message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::Execution(ExecutionErrorKind::Generic),
            message,
            interpreter.intern_symbol(SYMBOL_NAME_GENERIC_EXECUTION_ERROR)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_final_cause_works() {
        let mut interpreter = Interpreter::new();

        let cause_cause_error = Error::invalid_argument_count(&mut interpreter, "r");
        let cause_error = Error::invalid_argument_caused(&mut interpreter, "r", cause_cause_error);
        let error = Error::generic_execution_error_caused(&mut interpreter, "r", cause_error);

        assert!(
            match error.get_total_cause().get_error_kind() {
                ErrorKind::Argument(ArgumentErrorKind::InvalidArgumentCount) => true,
                _ => false
            }
        );
    }
}
