use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn boolean(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `to:boolean' takes one argument exactly."
        ).into_result();
    }

    let mut values = values;

    let result = library::is_truthy(
        interpreter,
        values.remove(0)
    )?;

    Ok(Value::Boolean(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_boolean() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(to:boolean 1)", Value::Boolean(true)),
            ("(to:boolean 1.1)", Value::Boolean(true)),
            ("(to:boolean #t)", Value::Boolean(true)),
            ("(to:boolean #f)", Value::Boolean(false)),
            ("(to:boolean \"string\")", Value::Boolean(true)),
            ("(to:boolean 'symbol)", Value::Boolean(true)),
            ("(to:boolean :keyword)", Value::Boolean(true)),
            ("(to:boolean nil)", Value::Boolean(false)),
            ("(to:boolean '())", Value::Boolean(false)),
            ("(to:boolean '(1 2 3))", Value::Boolean(true)),
            ("(to:boolean {})", Value::Boolean(true)),
            ("(to:boolean #())", Value::Boolean(true)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(to:boolean)",
            "(to:boolean 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
