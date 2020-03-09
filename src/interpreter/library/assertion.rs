use crate::interpreter::value::Value;
use crate::interpreter::error::*;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn assert_deep_equal(interpreter: &Interpreter, value1: Value, value2: Value) {
    assert!(
        library::deep_equal(
            interpreter,
            value1,
            value2
        ).unwrap()
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

pub fn assert_is_error<V, E>(error: &Result<V, E>) {
    assert!(error.is_err());
}

macro_rules! make_assertion_function {
    ($name:ident, $error_kind:pat) => {
        pub fn $name<T>(error: &Result<T, Error>) {
            assert!(error.is_err());

            let error = error.as_ref().err().unwrap().get_total_cause();

            assert!(
                match error.get_error_kind() {
                    $error_kind => true,
                    _ => false
                }
            );
        }
    }
}

make_assertion_function!(
    assert_invalid_argument_error,
    ErrorKind::InvalidArgument
);

make_assertion_function!(
    assert_invalid_argument_count_error,
    ErrorKind::InvalidArgumentCount
);

make_assertion_function!(
    assert_overflow_error,
    ErrorKind::Overflow
);

make_assertion_function!(
    assert_zero_division_error,
    ErrorKind::ZeroDivision
);

make_assertion_function!(
    assert_assertion_error,
    ErrorKind::Assertion
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

pub fn assert_results_are_equal(interpreter: &mut Interpreter, pairs: Vec<(&str, &str)>) {
    for (code, code_expected) in pairs {
        let expected = interpreter.execute(code_expected).unwrap();
        let result = interpreter.execute(code).unwrap();

        println!("{}", code_expected);
        interpreter.print_value(expected);
        println!();
        interpreter.print_value(result);
        println!();

        assert_deep_equal(
            interpreter,
            expected,
            result
        );
    }
}

pub fn assert_results_are_correct(interpreter: &mut Interpreter, pairs: Vec<(&str, Value)>) {
    for (code, expected) in pairs {
        let result = interpreter.execute(code).unwrap();

//        println!("{}", code);
//        println!("{:?}", expected);
//        println!("{:?}", result);

        assert_deep_equal(
            interpreter,
            expected,
            result
        );
    }
}

pub fn assert_results_are_errors(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>,
    error_kind: ErrorKind
) {
    for code in code_vector {
        let error = interpreter.execute(code).err().unwrap();
        let total_cause = error.get_total_cause();

        assert_eq!(error_kind, total_cause.get_error_kind());
    }
}

pub fn assert_results_are_functions(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>
) {
    for code in code_vector {
        let result = interpreter.execute(code);

        assert_is_function(result.unwrap())
    }
}

pub fn assert_results_are_just_errors(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>
) {
    for code in code_vector {
        let result = interpreter.execute(code);

        assert_is_error(&result);
    }
}

pub fn assert_results_are_invalid_argument_errors(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>
) {
    assert_results_are_errors(interpreter, code_vector, ErrorKind::InvalidArgument)
}

pub fn assert_results_are_invalid_argument_count_errors(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>
) {
    assert_results_are_errors(interpreter, code_vector, ErrorKind::InvalidArgumentCount)
}

pub fn assert_results_are_zero_division_errors(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>
) {
    assert_results_are_errors(interpreter, code_vector, ErrorKind::ZeroDivision)
}

pub fn assert_results_are_overflow_errors(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>
) {
    assert_results_are_errors(interpreter, code_vector, ErrorKind::Overflow)
}
