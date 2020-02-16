use crate::interpreter::value::Value;
use crate::interpreter::error::*;
use crate::interpreter::interpreter::Interpreter;

pub fn assert_deep_equal(interpreter: &mut Interpreter, value1: Value, value2: Value) {
    assert!(
        interpreter.deep_equal(value1, value2).unwrap()
    );
}

pub fn assert_vectors_deep_equal(interpreter: &mut Interpreter, values1: Vec<Value>, values2: Vec<Value>) {
    let mut values1 = values1;
    let mut values2 = values2;

    assert_eq!(values1.len(), values2.len());

    while values1.len() > 0 {
        let value1 = values1.remove(0);
        let value2 = values2.remove(0);

        assert_deep_equal(interpreter, value1, value2);
    }
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

            // todo: fix that, maybe that unnecessary
            //assert_eq!($symbol_name, error.get_symbol().get_name());
        }
    }
}

make_assertion_function!(
    assert_invalid_argument_error,
    ErrorKind::InvalidArgument,
    SYMBOL_NAME_INVALID_ARGUMENT_ERROR
);

make_assertion_function!(
    assert_invalid_argument_count_error,
    ErrorKind::InvalidArgumentCount,
    SYMBOL_NAME_INVALID_ARGUMENT_COUNT_ERROR
);

make_assertion_function!(
    assert_overflow_error,
    ErrorKind::Overflow,
    SYMBOL_NAME_OVERFLOW_ERROR
);

make_assertion_function!(
    assert_zero_division_error,
    ErrorKind::ZeroDivision,
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

pub fn assert_is_nil(interpreter: &mut Interpreter, param: Value) {
    assert!(
        match param {
            Value::Symbol(symbol) => {
                interpreter
                    .get_symbol(symbol)
                    .unwrap()
                    .is_nil()
            },
            _ => false
        }
    );
}
