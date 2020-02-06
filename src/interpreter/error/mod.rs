pub mod assertion;

use crate::interpreter::symbol::Symbol;
use crate::interpreter::interpreter::Interpreter;

pub const SYMBOL_NAME_GENERIC_EXECUTION_ERROR: &'static str = "generic-execution-error";
pub const SYMBOL_NAME_INVALID_ARGUMENT: &'static str = "invalid-argument";
pub const SYMBOL_NAME_INVALID_ARGUMENT_COUNT: &'static str = "invalid-argument-count";
pub const SYMBOL_NAME_OVERFLOW_ERROR: &'static str = "overflow-error";

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    Empty,

    GenericError,

    GenericExecutionError,
    OverflowError,

    VariableNotFound,
    FunctionNotFound,
    VariableAlreadyDefined,
    FunctionAlreadyDefined,

    InvalidArgument,
    InvalidArgumentCount,
}

#[derive(Clone, Debug)]
pub struct Error {
    error_kind: ErrorKind,
    message: String,
    caused_by: Option<Box<Error>>,
    symbol: Symbol,
}

impl Error {
    pub fn get_error_kind(&self) -> ErrorKind {
        self.error_kind
    }

    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }

    pub fn get_message(&self) -> &String {
        &self.message
    }

    pub fn get_total_cause(&self) -> &Error {
        match &self.caused_by {
            Some(b) => b.get_total_cause(),
            None => self
        }
    }
}

macro_rules! make_error_constructor {
    ($name:ident, $error_kind:expr, $symbol_name:expr) => {
        pub fn $name(interpreter: &mut Interpreter, message: &str) -> Error {
            Error::from(
                None,
                $error_kind,
                message,
                interpreter.intern_symbol($symbol_name)
            )
        }
    }
}

macro_rules! make_caused_error_constructor {
    ($name:ident, $error_kind:expr, $symbol_name:expr) => {
        pub fn $name(interpreter: &mut Interpreter, message: &str, cause: Error) -> Error {
            Error::from(
                Some(cause),
                $error_kind,
                message,
                interpreter.intern_symbol($symbol_name)
            )
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
            symbol: symbol
        }
    }

    pub fn empty() -> Error {
        Error {
            error_kind: ErrorKind::Empty,
            message: String::from(""),
            caused_by: None,
            symbol: Symbol::make_nil()
        }
    }

    pub fn generic_error(symbol: Symbol, message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::GenericError,
            message,
            symbol
        )
    }

    make_error_constructor!(
        invalid_argument,
        ErrorKind::InvalidArgument,
        SYMBOL_NAME_INVALID_ARGUMENT
    );
    make_caused_error_constructor!(
        invalid_argument_caused,
        ErrorKind::InvalidArgument,
        SYMBOL_NAME_INVALID_ARGUMENT
    );

    make_error_constructor!(
        invalid_argument_count,
        ErrorKind::InvalidArgumentCount,
        SYMBOL_NAME_INVALID_ARGUMENT_COUNT
    );
    make_caused_error_constructor!(
        invalid_argument_count_caused,
        ErrorKind::InvalidArgumentCount,
        SYMBOL_NAME_INVALID_ARGUMENT_COUNT
    );

    make_error_constructor!(
        generic_execution_error,
        ErrorKind::GenericExecutionError,
        SYMBOL_NAME_GENERIC_EXECUTION_ERROR
    );
    make_caused_error_constructor!(
        generic_execution_error_caused,
        ErrorKind::GenericExecutionError,
        SYMBOL_NAME_GENERIC_EXECUTION_ERROR
    );

    make_error_constructor!(
        overflow_error,
        ErrorKind::OverflowError,
        SYMBOL_NAME_OVERFLOW_ERROR
    );
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
                ErrorKind::InvalidArgumentCount => true,
                _ => false
            }
        );
    }
}
