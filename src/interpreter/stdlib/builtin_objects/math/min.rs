use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn min(
    _interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() == 0 {
        return Error::invalid_argument_count_error(
            "Built-in function `math:min' takes one argument at least.",
        )
        .into();
    }

    let mut values = values;

    let mut min = values[0];

    while values.len() > 0 {
        match (min, values.remove(0)) {
            (Value::Integer(int1), Value::Integer(int2)) => {
                if int1 > int2 {
                    min = Value::Integer(int2)
                }
            }
            (Value::Integer(int1), Value::Float(float2)) => {
                if (int1 as f64) > float2 {
                    min = Value::Float(float2)
                }
            }
            (Value::Float(float1), Value::Integer(int2)) => {
                if float1 > (int2 as f64) {
                    min = Value::Integer(int2)
                }
            }
            (Value::Float(float1), Value::Float(float2)) => {
                if float1 > float2 {
                    min = Value::Float(float2)
                }
            }
            _ => {
                return Error::invalid_argument_error(
                    "Built-in function `math:min' takes only integer or float arguments",
                )
                .into()
            }
        }
    }

    Ok(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_minimal_value() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(math:min 1)", Value::Integer(1)),
            ("(math:min 1.1)", Value::Float(1.1)),
            ("(math:min 1 2)", Value::Integer(1)),
            ("(math:min 1 2.0)", Value::Integer(1)),
            ("(math:min 1.0 2)", Value::Float(1.0)),
            ("(math:min 1.0 2.0)", Value::Float(1.0)),
            ("(math:min 1 2.0 3.0)", Value::Integer(1)),
            ("(math:min 1.0 2.0 3.0)", Value::Float(1.0)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(math:min)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(math:min #t)",
            "(math:min #f)",
            "(math:min 'symbol)",
            "(math:min \"string\")",
            "(math:min :keyword)",
            "(math:min '(s-expression))",
            "(math:min {})",
            "(math:min (function (lambda () 1)))",
            "(math:min (function (macro () 1)))",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
