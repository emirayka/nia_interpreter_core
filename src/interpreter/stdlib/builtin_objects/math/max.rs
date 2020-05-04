use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn max(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() == 0 {
        return Error::invalid_argument_count_error(
            "Built-in function `math:max' takes one argument at least.",
        )
        .into();
    }

    let mut values = values;

    let mut max = values[0];

    while values.len() > 0 {
        match (max, values.remove(0)) {
            (Value::Integer(int1), Value::Integer(int2)) => {
                if int1 < int2 {
                    max = Value::Integer(int2)
                }
            }
            (Value::Integer(int1), Value::Float(float2)) => {
                if (int1 as f64) < float2 {
                    max = Value::Float(float2)
                }
            }
            (Value::Float(float1), Value::Integer(int2)) => {
                if float1 < (int2 as f64) {
                    max = Value::Integer(int2)
                }
            }
            (Value::Float(float1), Value::Float(float2)) => {
                if float1 < float2 {
                    max = Value::Float(float2)
                }
            }
            _ => {
                return Error::invalid_argument_error(
                    "Built-in function `math:max' takes only integer or float arguments",
                )
                .into()
            }
        }
    }

    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_max_value() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(math:max 1)", Value::Integer(1)),
            ("(math:max 1.1)", Value::Float(1.1)),
            ("(math:max 1 2)", Value::Integer(2)),
            ("(math:max 1 2.0)", Value::Float(2.0)),
            ("(math:max 1.0 2)", Value::Integer(2)),
            ("(math:max 1.0 2.0)", Value::Float(2.0)),
            ("(math:max 1.0 2.0 3)", Value::Integer(3)),
            ("(math:max 1.0 2.0 3.0)", Value::Float(3.0)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(math:max)"];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(math:max #t)",
            "(math:max #f)",
            "(math:max 'symbol)",
            "(math:max \"string\")",
            "(math:max :keyword)",
            "(math:max '(s-expression))",
            "(math:max {})",
            "(math:max (function (lambda () 1)))",
            "(math:max (function (macro () 1)))",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }
}
