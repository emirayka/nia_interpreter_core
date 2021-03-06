use std::fmt;

pub const SYMBOL_NAME_FAILURE: &'static str = "failure";

pub const SYMBOL_NAME_PARSE_ERROR: &'static str = "parse-error";
pub const SYMBOL_NAME_GENERIC_EXECUTION_ERROR: &'static str =
    "generic-execution-error";
pub const SYMBOL_NAME_OVERFLOW_ERROR: &'static str = "overflow-error";
pub const SYMBOL_NAME_ZERO_DIVISION_ERROR: &'static str = "zero-division-error";
pub const SYMBOL_NAME_INVALID_CONS_ERROR: &'static str = "invalid-cons-error";

pub const SYMBOL_NAME_INVALID_ARGUMENT_ERROR: &'static str =
    "invalid-argument-error";
pub const SYMBOL_NAME_INVALID_ARGUMENT_COUNT_ERROR: &'static str =
    "invalid-argument-count-error";

pub const SYMBOL_NAME_STACK_OVERFLOW_ERROR: &'static str = "stack-overflow";
pub const SYMBOL_NAME_ASSERTION_ERROR: &'static str = "assertion-error";
pub const SYMBOL_NAME_BREAK_ERROR: &'static str = "break-error";
pub const SYMBOL_NAME_CONTINUE_ERROR: &'static str = "continue-error";

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

    StackOverflow,
    Assertion,
    Break,
    Continue,
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
            None => self,
        }
    }

    pub fn is_failure(&self) -> bool {
        self.get_total_cause().get_error_kind() == ErrorKind::Failure
    }
}

impl Error {
    pub fn from<S>(
        caused_by: Option<Error>,
        kind: ErrorKind,
        message: S,
        symbol_name: String,
    ) -> Error
    where
        S: Into<String>,
    {
        Error {
            error_kind: kind,
            message: message.into(),
            caused_by: match caused_by {
                Some(error) => Some(Box::new(error)),
                None => None,
            },
            symbol_name,
        }
    }

    pub fn failure<S>(message: S) -> Error
    where
        S: Into<String>,
    {
        Error {
            error_kind: ErrorKind::Failure,
            message: message.into(),
            caused_by: None,
            symbol_name: String::from(SYMBOL_NAME_FAILURE),
        }
    }

    pub fn parse_error<S>(message: S) -> Error
    where
        S: Into<String>,
    {
        Error::from(
            None,
            ErrorKind::ParseError,
            message,
            String::from(SYMBOL_NAME_PARSE_ERROR),
        )
    }

    pub fn generic_error<S>(symbol_name: String, message: S) -> Error
    where
        S: Into<String>,
    {
        Error::from(None, ErrorKind::GenericError, message.into(), symbol_name)
    }

    pub fn generic_execution_error<S>(message: S) -> Error
    where
        S: Into<String>,
    {
        Error::from(
            None,
            ErrorKind::GenericExecution,
            message,
            String::from(SYMBOL_NAME_GENERIC_EXECUTION_ERROR),
        )
    }
    pub fn generic_execution_error_caused<S>(message: S, cause: Error) -> Error
    where
        S: Into<String>,
    {
        Error::from(
            Some(cause),
            ErrorKind::GenericExecution,
            message,
            String::from(SYMBOL_NAME_GENERIC_EXECUTION_ERROR),
        )
    }

    pub fn overflow_error<S>(message: S) -> Error
    where
        S: Into<String>,
    {
        Error::from(
            None,
            ErrorKind::Overflow,
            message,
            String::from(SYMBOL_NAME_OVERFLOW_ERROR),
        )
    }

    pub fn overflow_error_caused<S>(message: S, cause: Error) -> Error
    where
        S: Into<String>,
    {
        Error::from(
            Some(cause),
            ErrorKind::Overflow,
            message,
            String::from(SYMBOL_NAME_OVERFLOW_ERROR),
        )
    }

    pub fn zero_division_error<S>(message: S) -> Error
    where
        S: Into<String>,
    {
        Error::from(
            None,
            ErrorKind::ZeroDivision,
            message,
            String::from(SYMBOL_NAME_ZERO_DIVISION_ERROR),
        )
    }

    pub fn zero_division_error_caused<S>(message: S, cause: Error) -> Error
    where
        S: Into<String>,
    {
        Error::from(
            Some(cause),
            ErrorKind::ZeroDivision,
            message,
            String::from(SYMBOL_NAME_ZERO_DIVISION_ERROR),
        )
    }

    pub fn invalid_cons_error<S>(message: S) -> Error
    where
        S: Into<String>,
    {
        Error::from(
            None,
            ErrorKind::InvalidCons,
            message,
            String::from(SYMBOL_NAME_INVALID_CONS_ERROR),
        )
    }

    pub fn invalid_cons_error_caused<S>(message: S, cause: Error) -> Error
    where
        S: Into<String>,
    {
        Error::from(
            Some(cause),
            ErrorKind::InvalidCons,
            message,
            String::from(SYMBOL_NAME_INVALID_CONS_ERROR),
        )
    }

    pub fn invalid_argument_error<S>(message: S) -> Error
    where
        S: Into<String>,
    {
        Error::from(
            None,
            ErrorKind::InvalidArgument,
            message,
            String::from(SYMBOL_NAME_INVALID_ARGUMENT_ERROR),
        )
    }
    pub fn invalid_argument_error_caused<S>(message: S, cause: Error) -> Error
    where
        S: Into<String>,
    {
        Error::from(
            Some(cause),
            ErrorKind::InvalidArgument,
            message,
            String::from(SYMBOL_NAME_INVALID_ARGUMENT_ERROR),
        )
    }

    pub fn invalid_argument_count_error<S>(message: S) -> Error
    where
        S: Into<String>,
    {
        Error::from(
            None,
            ErrorKind::InvalidArgumentCount,
            message,
            String::from(SYMBOL_NAME_INVALID_ARGUMENT_COUNT_ERROR),
        )
    }

    pub fn invalid_argument_count_error_caused<S>(
        message: S,
        cause: Error,
    ) -> Error
    where
        S: Into<String>,
    {
        Error::from(
            Some(cause),
            ErrorKind::InvalidArgumentCount,
            message,
            String::from(SYMBOL_NAME_INVALID_ARGUMENT_COUNT_ERROR),
        )
    }

    pub fn stack_overflow_error() -> Error {
        Error::from(
            None,
            ErrorKind::StackOverflow,
            "",
            String::from(SYMBOL_NAME_STACK_OVERFLOW_ERROR),
        )
    }

    pub fn assertion_error<S>(message: S) -> Error
    where
        S: Into<String>,
    {
        Error::from(
            None,
            ErrorKind::Assertion,
            message,
            String::from(SYMBOL_NAME_ASSERTION_ERROR),
        )
    }

    pub fn break_error() -> Error {
        Error::from(
            None,
            ErrorKind::Break,
            "",
            String::from(SYMBOL_NAME_BREAK_ERROR),
        )
    }

    pub fn continue_error() -> Error {
        Error::from(
            None,
            ErrorKind::Continue,
            "",
            String::from(SYMBOL_NAME_CONTINUE_ERROR),
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} \"{}\")", self.symbol_name, self.message)
            .expect("Error: Failed writing.");

        if let Some(cause) = &self.caused_by {
            let cause_error = cause.as_ref();

            write!(f, " caused by:")?;
            write!(f, "\n")?;
            cause_error.fmt(f)
        } else {
            write!(f, "\n")
        }
    }
}

impl<T> From<Error> for Result<T, Error> {
    fn from(v: Error) -> Self {
        Err(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::interpreter::interpreter::Interpreter;

    #[test]
    fn final_cause_works() {
        let _interpreter = Interpreter::new();

        let cause_cause_error = Error::invalid_argument_count_error("r");
        let cause_error =
            Error::invalid_argument_count_error_caused("r", cause_cause_error);
        let error = Error::generic_execution_error_caused("r", cause_error);

        nia_assert(match error.get_total_cause().get_error_kind() {
            ErrorKind::InvalidArgumentCount => true,
            _ => false,
        });
    }
}
