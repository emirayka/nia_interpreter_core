use crate::interpreter::value::Value;
use crate::interpreter::error::*;
use crate::interpreter::interpreter::Interpreter;

pub fn assert_deep_equal(interpreter: &mut Interpreter, value1: &Value, value2: &Value) {
    assert!(
        interpreter.deep_equal(value1, value2)
    );
}

pub fn assert_error<V, E>(error: &Result<V, E>) {
    assert!(error.is_err());
}

macro_rules! make_assertion_function {
    ($name:ident, $error_kind:pat, $symbol_name:expr) => {
        pub fn $name(error: &Result<Value, Error>) {
            assert!(error.is_err());

            let error = error.as_ref().err().unwrap().get_total_cause();

            assert!(
                match error.get_error_kind() {
                    $error_kind => true,
                    _ => false
                }
            );

            assert_eq!($symbol_name, error.get_symbol().get_name());
        }
    }
}

make_assertion_function!(
    assert_invalid_argument_error,
    ErrorKind::InvalidArgument,
    SYMBOL_NAME_INVALID_ARGUMENT
);

make_assertion_function!(
    assert_invalid_argument_count_error,
    ErrorKind::InvalidArgumentCount,
    SYMBOL_NAME_INVALID_ARGUMENT_COUNT
);

make_assertion_function!(
    assert_overflow_error,
    ErrorKind::OverflowError,
    SYMBOL_NAME_OVERFLOW_ERROR
);

make_assertion_function!(
    assert_zero_division_error,
    ErrorKind::ZeroDivisionError,
    SYMBOL_NAME_ZERO_DIVISION_ERROR
);

pub fn assert_is_function(param: Value) {
    assert!(
        match param {
            Value::Function(_) => true,
            _ => false
        }
    );
}

pub fn assert_is_object(param: Value) {
    assert!(
        match param {
            Value::Object(_) => true,
            _ => false
        }
    );
}

pub fn assert_is_nil(param: Value) {
    assert!(
        match param {
            Value::Symbol(symbol) => symbol.is_nil(),
            _ => false
        }
    );
}
