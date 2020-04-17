use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

pub fn int(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `to:int' takes one argument exactly."
        ).into_result();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Integer(int) => Value::Integer(int),
        Value::Float(float) => Value::Integer(float as i64),
        Value::Boolean(true) => Value::Integer(1),
        Value::Boolean(false) => Value::Integer(0),
        _ => return Error::generic_execution_error(
            "Only integers, floats or booleans can be converted to int."
        ).into_result()
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_integer() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(to:int 1)", Value::Integer(1)),
            ("(to:int 1.1)", Value::Integer(1)),
            ("(to:int 1.9)", Value::Integer(1)),
            ("(to:int #t)", Value::Integer(1)),
            ("(to:int #f)", Value::Integer(0)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_generic_execution_error_when_invalid_conversion() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            "(to:int \"string\")",
            "(to:int 'symbol)",
            "(to:int :keyword)",
            "(to:int '(1 2 3))",
            "(to:int {})",
            "(to:int #())",
        );

        assertion::assert_results_are_generic_execution_errors(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(to:int)",
            "(to:int 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
