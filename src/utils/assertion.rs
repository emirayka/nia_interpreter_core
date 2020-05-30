#[allow(unused_imports)]
use nia_basic_assertions::*;

use crate::interpreter::parse;
use crate::interpreter::read_element;

use crate::Error;
use crate::ErrorKind;
use crate::Interpreter;
use crate::Value;

use crate::SYMBOL_NAME_GENERIC_EXECUTION_ERROR;
use crate::SYMBOL_NAME_INVALID_ARGUMENT_COUNT_ERROR;
use crate::SYMBOL_NAME_INVALID_ARGUMENT_ERROR;
use crate::SYMBOL_NAME_OVERFLOW_ERROR;
use crate::SYMBOL_NAME_ZERO_DIVISION_ERROR;

use crate::library;

pub fn assert_deep_equal(
    interpreter: &Interpreter,
    value1: Value,
    value2: Value,
) {
    nia_assert_is_ok(&library::print_value(interpreter, value1));
    nia_assert_is_ok(&library::print_value(interpreter, value2));
    nia_assert(library::deep_equal(interpreter, value1, value2).unwrap());
}

pub fn assert_parsing_reading_result_is_correct(
    interpreter: &mut Interpreter,
    expected: Value,
    code: &str,
) {
    let elements = parse(code).unwrap();
    let first_element = elements.get_elements().remove(0);

    let result = read_element(interpreter, first_element).unwrap();

    crate::utils::assert_deep_equal(interpreter, expected, result);
}

pub fn assert_option_deep_equal(
    interpreter: &Interpreter,
    value1: Option<Value>,
    value2: Option<Value>,
) {
    match (value1, value2) {
        (Some(v1), Some(v2)) => {
            nia_assert(library::deep_equal(interpreter, v1, v2).unwrap())
        }
        (None, None) => {}
        _ => {
            nia_assert(false);
        }
    }
}

pub fn assert_vectors_deep_equal(
    interpreter: &mut Interpreter,
    values1: Vec<Value>,
    values2: Vec<Value>,
) {
    let mut values1 = values1;
    let mut values2 = values2;

    nia_assert_equal(values1.len(), values2.len());

    while values1.len() > 0 {
        let value1 = values1.remove(0);
        let value2 = values2.remove(0);

        assert_deep_equal(interpreter, value1, value2);
    }
}

macro_rules! make_assertion_function {
    ($name:ident, $error_kind:pat) => {
        pub fn $name<T>(error: &Result<T, Error>) {
            nia_assert(error.is_err());

            let error = error.as_ref().err().unwrap().get_total_cause();

            nia_assert(match error.get_error_kind() {
                $error_kind => true,
                _ => false,
            });
        }
    };
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
    assert_generic_execution_error,
    ErrorKind::GenericExecution
);
make_assertion_function!(assert_overflow_error, ErrorKind::Overflow);
make_assertion_function!(assert_zero_division_error, ErrorKind::ZeroDivision);
make_assertion_function!(assert_assertion_error, ErrorKind::Assertion);
make_assertion_function!(assert_stack_overflow_error, ErrorKind::StackOverflow);

pub fn assert_is_function(param: Value) {
    nia_assert(match param {
        Value::Function(_) => true,
        _ => false,
    });
}

pub fn assert_is_object(param: Value) {
    nia_assert(match param {
        Value::Object(_) => true,
        _ => false,
    });
}

pub fn assert_is_nil(interpreter: &mut Interpreter, param: Value) {
    nia_assert(match param {
        Value::Symbol(symbol_id) => {
            interpreter.symbol_is_nil(symbol_id).unwrap()
        }
        _ => false,
    });
}

pub fn assert_results_are_equal(
    interpreter: &mut Interpreter,
    pairs: Vec<(&str, &str)>,
) {
    for (code, code_expected) in pairs {
        let expected = interpreter
            .execute_in_main_environment(code_expected)
            .unwrap();
        let result = interpreter.execute_in_main_environment(code).unwrap();

        println!("{}", code_expected);
        nia_assert_is_ok(&crate::library::print_value(interpreter, expected));
        nia_assert_is_ok(&crate::library::print_value(interpreter, result));

        assert_deep_equal(interpreter, expected, result);
    }
}

pub fn assert_results_are_correct(
    interpreter: &mut Interpreter,
    pairs: Vec<(&str, Value)>,
) {
    for (code, expected) in pairs {
        let result = interpreter.execute_in_main_environment(code).unwrap();

        println!("{}", code);
        nia_assert_is_ok(&crate::library::print_value(interpreter, expected));
        println!();
        nia_assert_is_ok(&crate::library::print_value(interpreter, result));
        println!();

        assert_deep_equal(interpreter, expected, result);
    }
}

pub fn assert_results_are_errors(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>,
    error_kind: ErrorKind,
    symbol_name: &str,
) {
    for code in code_vector {
        println!("{}", code);

        let error =
            interpreter.execute_in_main_environment(code).err().unwrap();
        let total_cause = error.get_total_cause();

        if total_cause.get_error_kind() == error_kind {
            nia_assert_equal(symbol_name, total_cause.get_symbol_name());
        } else if total_cause.get_error_kind() == ErrorKind::GenericError {
            nia_assert_equal(symbol_name, total_cause.get_symbol_name());
        } else {
            panic!();
        }
    }
}

pub fn assert_results_are_functions(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>,
) {
    for code in code_vector {
        let result = interpreter.execute_in_main_environment(code);

        assert_is_function(result.unwrap())
    }
}

pub fn assert_results_are_just_errors(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>,
) {
    for code in code_vector {
        let result = interpreter.execute_in_main_environment(code);

        nia_assert_is_err(&result);
    }
}

pub fn assert_results_are_generic_execution_errors(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>,
) {
    assert_results_are_errors(
        interpreter,
        code_vector,
        ErrorKind::GenericExecution,
        SYMBOL_NAME_GENERIC_EXECUTION_ERROR,
    )
}

pub fn assert_results_are_invalid_argument_errors(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>,
) {
    assert_results_are_errors(
        interpreter,
        code_vector,
        ErrorKind::InvalidArgument,
        SYMBOL_NAME_INVALID_ARGUMENT_ERROR,
    )
}

pub fn assert_results_are_invalid_argument_count_errors(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>,
) {
    assert_results_are_errors(
        interpreter,
        code_vector,
        ErrorKind::InvalidArgumentCount,
        SYMBOL_NAME_INVALID_ARGUMENT_COUNT_ERROR,
    )
}

pub fn assert_results_are_zero_division_errors(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>,
) {
    assert_results_are_errors(
        interpreter,
        code_vector,
        ErrorKind::ZeroDivision,
        SYMBOL_NAME_ZERO_DIVISION_ERROR,
    )
}

pub fn assert_results_are_overflow_errors(
    interpreter: &mut Interpreter,
    code_vector: Vec<&str>,
) {
    assert_results_are_errors(
        interpreter,
        code_vector,
        ErrorKind::Overflow,
        SYMBOL_NAME_OVERFLOW_ERROR,
    )
}
