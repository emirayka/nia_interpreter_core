use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn sub(
    _interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() < 1 || values.len() > 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `-' must take exactly two argument",
        )
        .into();
    }

    let mut values = values;

    let result = if values.len() == 2 {
        match (values.remove(0), values.remove(0)) {
            (Value::Integer(int1), Value::Integer(int2)) => {
                match int1.checked_sub(int2) {
                    Some(int_result) => Value::Integer(int_result),
                    None => {
                        return Error::overflow_error(&format!(
                        "Attempt to subtract values {} {} leads to overflow",
                        int1, int2
                    ))
                        .into()
                    }
                }
            }
            (Value::Integer(int1), Value::Float(float2)) => {
                Value::Float((int1 as f64) - float2)
            }
            (Value::Float(float1), Value::Integer(int2)) => {
                Value::Float(float1 - (int2 as f64))
            }
            (Value::Float(float1), Value::Float(float2)) => {
                Value::Float(float1 - float2)
            }
            _ => {
                return Error::invalid_argument_error(
                    "Built-in function `-' takes only integers or float.",
                )
                .into();
            }
        }
    } else {
        match values.remove(0) {
            Value::Integer(int) => Value::Integer(-int),
            Value::Float(float) => Value::Float(-float),
            _ => {
                return Error::invalid_argument_error(
                    "Built-in function `-' takes only integers or float.",
                )
                .into();
            }
        }
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_correct_subtraction_of_two_integers() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(- 1)", Value::Integer(-1)),
            ("(- 1 2)", Value::Integer(-1)),
            ("(- 1.0)", Value::Float(-1.0)),
            ("(- 1 2.0)", Value::Float(-1.0)),
            ("(- 1.0 2)", Value::Float(-1.0)),
            ("(- 1.0 2.0)", Value::Float(-1.0)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_count_when_invalid_count_of_arguments_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(-)", "(- 1 2 3)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(- 1 #t)",
            "(- 1 #f)",
            "(- 1 'symbol)",
            "(- 1 \"string\")",
            "(- 1 :keyword)",
            "(- 1 '(s-expression))",
            "(- 1 {})",
            "(- 1 (function (lambda () 1)))",
            "(- 1 (function (macro () 1)))",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_overflow_error_when_an_overflow_occurred() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(- 9223372036854775800 -10)",
            "(- 10 -9223372036854775800)",
            "(- -10 9223372036854775800)",
            "(- -9223372036854775800 10)",
        ];

        utils::assert_results_are_overflow_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
