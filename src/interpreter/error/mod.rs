pub const SYMBOL_NAME_FAILURE: &'static str = "failure";

pub const SYMBOL_NAME_PARSE_ERROR: &'static str = "parse";
pub const SYMBOL_NAME_GENERIC_EXECUTION_ERROR: &'static str = "generic-execution-error";
pub const SYMBOL_NAME_OVERFLOW_ERROR: &'static str = "overflow-error";
pub const SYMBOL_NAME_ZERO_DIVISION_ERROR: &'static str = "zero-division-error";
pub const SYMBOL_NAME_INVALID_CONS_ERROR: &'static str = "invalid-cons-error";

pub const SYMBOL_NAME_INVALID_ARGUMENT_ERROR: &'static str = "invalid-argument-error";
pub const SYMBOL_NAME_INVALID_ARGUMENT_COUNT_ERROR: &'static str = "invalid-argument-count-error";

pub const SYMBOL_NAME_ASSERTION_ERROR: &'static str = "assertion";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    Failure,

    ParseError,

    GenericError,

    GenericExecution,
    Overflow,
    ZeroDivision,
    InvalidCons,

    InvalidArgument,
    InvalidArgumentCount,
    Assertion,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    error_kind: ErrorKind,
    message: String,
    caused_by: Option<Box<Error>>,
    symbol_name: String,
}

impl Error {
    pub fn get_error_kind(&self) -> ErrorKind {
        self.error_kind
    }

    pub fn get_symbol_name(&self) -> &String {
        &self.symbol_name
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

    pub fn into_result<T>(self) -> Result<T, Error> {
        Err(self)
    }

    pub fn describe(&self) {
        print!("({} \"{}\")", self.symbol_name, self.message);

        if let Some(cause) = &self.caused_by {
            let cause_error = cause.as_ref();

            print!(" caused by:");
            println!();
            cause_error.describe();
        } else {
            println!();
        }
    }
}

impl Error {
    pub fn from(caused_by: Option<Error>, kind: ErrorKind, message: &str, symbol_name: String) -> Error {
        Error {
            error_kind: kind,
            message: String::from(message),
            caused_by: match caused_by {
                Some(error) => Some(Box::new(error)),
                None => None
            },
            symbol_name
        }
    }

    pub fn failure(message: String) -> Error {
        Error {
            error_kind: ErrorKind::Failure,
            message,
            caused_by: None,
            symbol_name: String::from("failure"),
        }
    }

    pub fn parse_error(message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::ParseError,
            message,
            String::from(SYMBOL_NAME_PARSE_ERROR)
        )
    }

    pub fn generic_error(symbol_name: String, message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::GenericError,
            message,
            symbol_name
        )
    }

    pub fn generic_execution_error(message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::GenericExecution,
            message,
            String::from(SYMBOL_NAME_GENERIC_EXECUTION_ERROR)
        )
    }
    pub fn generic_execution_error_caused(message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::GenericExecution,
            message,
            String::from(SYMBOL_NAME_GENERIC_EXECUTION_ERROR)
        )
    }

    pub fn overflow_error(message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::Overflow,
            message,
        String::from(SYMBOL_NAME_OVERFLOW_ERROR)
        )
    }

    pub fn overflow_error_caused(message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::Overflow,
            message,
            String::from(SYMBOL_NAME_OVERFLOW_ERROR)
        )
    }

    pub fn zero_division_error(message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::ZeroDivision,
            message,
            String::from(SYMBOL_NAME_ZERO_DIVISION_ERROR)
        )
    }

    pub fn zero_division_error_caused(message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::ZeroDivision,
            message,
            String::from(SYMBOL_NAME_ZERO_DIVISION_ERROR)
        )
    }

    pub fn invalid_cons_error(message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::InvalidCons,
            message,
            String::from(SYMBOL_NAME_INVALID_CONS_ERROR)
        )
    }

    pub fn invalid_cons_error_caused(message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::InvalidCons,
            message,
            String::from(SYMBOL_NAME_INVALID_CONS_ERROR)
        )
    }

    pub fn invalid_argument_error(message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::InvalidArgument,
            message,
            String::from(SYMBOL_NAME_INVALID_ARGUMENT_ERROR)
        )
    }
    pub fn invalid_argument_error_caused(message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::InvalidArgument,
            message,
            String::from(SYMBOL_NAME_INVALID_ARGUMENT_ERROR)
        )
    }

    pub fn invalid_argument_count_error(message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::InvalidArgumentCount,
            message,
        String::from(SYMBOL_NAME_INVALID_ARGUMENT_COUNT_ERROR)
        )
    }

    pub fn invalid_argument_count_error_caused(message: &str, cause: Error) -> Error {
        Error::from(
            Some(cause),
            ErrorKind::InvalidArgumentCount,
            message,
            String::from(SYMBOL_NAME_INVALID_ARGUMENT_COUNT_ERROR)
        )
    }

    pub fn assertion_error(message: &str) -> Error {
        Error::from(
            None,
            ErrorKind::Assertion,
            message,
            String::from(SYMBOL_NAME_ASSERTION_ERROR)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::interpreter::Interpreter;

    #[test]
    fn final_cause_works() {
        let interpreter = Interpreter::new();

        let cause_cause_error = interpreter.make_invalid_argument_count_error("r");
        let cause_error = interpreter.make_invalid_argument_count_error_caused(
            "r",
            cause_cause_error
        );
        let error = interpreter.make_generic_execution_error_caused(
            "r",
            cause_error
        );

        assert!(
            match error.get_total_cause().get_error_kind() {
                ErrorKind::InvalidArgumentCount => true,
                _ => false
            }
        );
    }
}
