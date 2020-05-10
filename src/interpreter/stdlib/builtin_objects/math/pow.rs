use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

fn positive_int_pow(a: i64, b: i64) -> Option<i64> {
    if b == 0 {
        return Some(1);
    } else if b == 1 {
        return Some(a);
    } else if b % 2 != 0 {
        let new_a = match a.checked_mul(a) {
            Some(new_a) => new_a,
            None => return None,
        };
        let new_b = (b - 1) / 2;
        match positive_int_pow(new_a, new_b) {
            Some(result) => a.checked_mul(result),
            None => None,
        }
    } else {
        let new_a = match a.checked_mul(a) {
            Some(new_a) => new_a,
            None => return None,
        };
        let new_b = b / 2;
        positive_int_pow(new_a, new_b)
    }
}

fn checked_int_pow(a: i64, b: i64) -> Option<Value> {
    if b >= 0 {
        match positive_int_pow(a, b) {
            Some(result) => Some(Value::Integer(result)),
            None => None,
        }
    } else {
        Some(Value::Float((a as f64).powf(b as f64)))
    }
}

pub fn pow(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `math:pow' must take exactly two argument",
        )
        .into();
    }

    let mut values = values;

    match (values.remove(0), values.remove(0)) {
        (Value::Integer(int1), Value::Integer(int2)) => {
            match checked_int_pow(int1, int2) {
                Some(value) => Ok(value),
                None => Error::overflow_error(&format!(
                    "Cannot compute pow of {} on {}",
                    int1, int2
                ))
                .into(),
            }
        },
        (Value::Integer(int1), Value::Float(float2)) => {
            Ok(Value::Float((int1 as f64).powf(float2)))
        },
        (Value::Float(float1), Value::Integer(int2)) => {
            Ok(Value::Float(float1.powf(int2 as f64)))
        },
        (Value::Float(float1), Value::Float(float2)) => {
            Ok(Value::Float(float1.powf(float2)))
        },
        _ => return Error::invalid_argument_error(
            "Built-in function `math:pow' must take either integers or float.",
        )
        .into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_correct_power_of_two_integers() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![("(math:pow 3 4)", Value::Integer(81))];

        utils::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_correct_float_power() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(math:pow 3 4.0)", Value::Float(81.0)),
            ("(math:pow 3.0 4)", Value::Float(81.0)),
            ("(math:pow 3.0 4.0)", Value::Float(81.0)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn should_be_able_to_handle_float_and_negative_values() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(math:pow 4 0.5)", Value::Float(2.0)),
            ("(math:pow 4 -1)", Value::Float(0.25)),
            ("(math:pow 2 -2)", Value::Float(0.25)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector =
            vec!["(math:pow)", "(math:pow 1)", "(math:pow 1 2 3)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(math:pow 1 #t)",
            "(math:pow 1 #f)",
            "(math:pow 1 'symbol)",
            "(math:pow 1 \"string\")",
            "(math:pow 1 :keyword)",
            "(math:pow 1 '(s-expression))",
            "(math:pow 1 {})",
            "(math:pow 1 (function (lambda () 1)))",
            "(math:pow 1 (function (macro () 1)))",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        )
    }

    #[test]
    fn returns_overflow_error_when_an_overflow_occurred() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(math:pow 2 65)", "(math:pow 4 33)"];

        utils::assert_results_are_overflow_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
