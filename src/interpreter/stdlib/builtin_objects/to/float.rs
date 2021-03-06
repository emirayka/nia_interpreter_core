use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn float(
    _interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `to:float' takes one argument exactly.",
        )
        .into();
    }

    let mut values = values;

    let result =
        match values.remove(0) {
            Value::Integer(int) => Value::Float(int as f64),
            Value::Float(float) => Value::Float(float),
            Value::Boolean(true) => Value::Float(1.0),
            Value::Boolean(false) => Value::Float(0.0),
            _ => return Error::generic_execution_error(
                "Only integers, floats or booleans can be converted to float.",
            )
            .into(),
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
    fn returns_integer() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(to:float 1)", Value::Float(1.0)),
            ("(to:float 1.1)", Value::Float(1.1)),
            ("(to:float 1.9)", Value::Float(1.9)),
            ("(to:float #t)", Value::Float(1.0)),
            ("(to:float #f)", Value::Float(0.0)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_generic_execution_error_when_invalid_conversion() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            "(to:float \"string\")",
            "(to:float 'symbol)",
            "(to:float :keyword)",
            "(to:float '(1 2 3))",
            "(to:float {})",
            "(to:float #())",
        ];

        utils::assert_results_are_generic_execution_errors(
            &mut interpreter,
            pairs,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(to:float)", "(to:float 1 2)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
