use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn abs(
    _interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `math:abs' must take exactly one argument.",
        )
        .into();
    }

    let mut values = values;

    match values.remove(0) {
        Value::Integer(int) => {
            if int < 0 {
                Ok(Value::Integer(-int))
            } else {
                Ok(Value::Integer(int))
            }
        }
        Value::Float(float) => {
            if float.is_sign_negative() {
                Ok(Value::Float(-float))
            } else {
                Ok(Value::Float(float))
            }
        }
        _ => {
            return Error::invalid_argument_error(
                "Built-in function `math:abs' must take only integer or float values.",
            )
            .into()
        }
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
    fn returns_abs_of_the_value() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(math:abs 1)", Value::Integer(1)),
            ("(math:abs -1)", Value::Integer(1)),
            ("(math:abs 1.1)", Value::Float(1.1)),
            ("(math:abs -1.1)", Value::Float(1.1)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(math:abs #t)",
            "(math:abs #f)",
            "(math:abs 'symbol)",
            "(math:abs \"string\")",
            "(math:abs :keyword)",
            "(math:abs '(s-expression))",
            "(math:abs {})",
            "(math:abs (function (lambda () 1)))",
            "(math:abs (function (macro () 1)))",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector =
            vec!["(math:abs)", "(math:abs 1 2)", "(math:abs 1 2 3)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
