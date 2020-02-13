use crate::interpreter::symbol::Symbol;

pub const SYMBOL_NAME_GENERIC_EXECUTION_ERROR: &'static str = "generic-execution-error";
pub const SYMBOL_NAME_OVERFLOW_ERROR: &'static str = "overflow-error";
pub const SYMBOL_NAME_ZERO_DIVISION_ERROR: &'static str = "zero-division-error";
pub const SYMBOL_NAME_INVALID_CONS_ERROR: &'static str = "invalid-cons-error";

pub const SYMBOL_NAME_INVALID_ARGUMENT_ERROR: &'static str = "invalid-argument-error";
pub const SYMBOL_NAME_INVALID_ARGUMENT_COUNT_ERROR: &'static str = "invalid-argument-count-error";

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    Empty,

    GenericError,

    GenericExecution,
    Overflow,
    ZeroDivision,
    InvalidCons,

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

impl Error {
    pub fn from(caused_by: Option<Error>, kind: ErrorKind, message: &str, symbol: Symbol) -> Error {
        Error {
            error_kind: kind,
            message: String::from(message),
            caused_by: match caused_by {
                Some(error) => Some(Box::new(error)),
                None => None
            },
            symbol
        }
    }

    pub fn empty(symbol: Symbol) -> Error {
        Error {
            error_kind: ErrorKind::Empty,
            message: String::from(""),
            caused_by: None,
            symbol
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

    pub fn generic_execution_error(symbol: Symbol, message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::GenericExecution,
            message,
            symbol
        )
    }
    pub fn generic_execution_error_caused(symbol: Symbol, message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::GenericExecution,
            message,
            symbol
        )
    }

    pub fn overflow_error(symbol: Symbol, message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::Overflow,
            message,
            symbol
        )
    }

    pub fn overflow_error_caused(symbol: Symbol, message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::Overflow,
            message,
            symbol
        )
    }

    pub fn zero_division_error(symbol: Symbol, message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::ZeroDivision,
            message,
            symbol
        )
    }

    pub fn zero_division_error_caused(symbol: Symbol, message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::ZeroDivision,
            message,
            symbol
        )
    }

    pub fn invalid_cons_error(symbol: Symbol, message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::InvalidCons,
            message,
            symbol
        )
    }

    pub fn invalid_cons_error_caused(symbol: Symbol, message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::InvalidCons,
            message,
            symbol
        )
    }

    pub fn invalid_argument_error(symbol: Symbol, message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::InvalidArgument,
            message,
            symbol
        )
    }
    pub fn invalid_argument_error_caused(symbol: Symbol, message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::InvalidArgument,
            message,
            symbol
        )
    }

    pub fn invalid_argument_count_error(symbol: Symbol, message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::InvalidArgumentCount,
            message,
            symbol
        )
    }
    pub fn invalid_argument_count_error_caused(symbol: Symbol, message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::InvalidArgumentCount,
            message,
            symbol
        )
    }
}

#[cfg(test)]
mod tests {
//    use super::*;

    // todo: fix
//    #[test]
//    fn test_final_cause_works() {
//        let mut interpreter = Interpreter::new();
//
//        let cause_cause_error = Error::invalid_argument_count(interpreter, "r");
//        let cause_error = Error::invalid_argument_caused(interpreter, "r", cause_cause_error);
//        let error = Error::generic_execution_error_caused(interpreter, "r", cause_error);
//
//        assert!(
//            match error.get_total_cause().get_error_kind() {
//                ErrorKind::InvalidArgumentCount => true,
//                _ => false
//            }
//        );
//    }
}
