use crate::interpreter::value::Value;
use crate::interpreter::error::*;

pub fn assert_error<V, E>(error: &Result<V, E>) {
    assert!(error.is_err());
}

pub fn assert_argument_error(error: &Result<Value, Error>) {
    assert!(error.is_err());

    let error = error.as_ref().err().unwrap().get_total_cause();

    assert!(
        match error.get_error_kind() {
            ErrorKind::InvalidArgument => true,
            ErrorKind::InvalidArgumentCount => true,
            _ => false
        }
    );
}

pub fn assert_invalid_argument_error(error: &Result<Value, Error>) {
    assert!(error.is_err());

    let error = error.as_ref().err().unwrap().get_total_cause();

    assert!(
        match error.get_error_kind() {
            ErrorKind::InvalidArgument => true,
            _ => false
        }
    );

    assert_eq!(SYMBOL_NAME_INVALID_ARGUMENT, error.get_symbol().unwrap().get_name());
}

pub fn assert_invalid_argument_count_error(error: &Result<Value, Error>) {
    assert!(error.is_err());

    let error = error.as_ref().err().unwrap().get_total_cause();

    assert!(
        match error.get_error_kind() {
            ErrorKind::InvalidArgumentCount => true,
            _ => false
        }
    );

    assert_eq!(SYMBOL_NAME_INVALID_ARGUMENT_COUNT, error.get_symbol().unwrap().get_name());
}

