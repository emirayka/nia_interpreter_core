use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn send_wait(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `action:send-wait' takes one argument exactly."
        ).into();
    }

    let mut values = values;

    let milliseconds = library::read_as_i64(
        interpreter,
        values.remove(0)
    )?;

    let text_type_symbol_value = interpreter.intern_symbol_value("wait");
    let text_type = interpreter.vec_to_list(vec!(
        text_type_symbol_value,
        Value::Integer(milliseconds)
    ));

    library::add_value_to_root_list(
        interpreter,
        "--actions",
        text_type
    )?;

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn adds_action_to_action_list() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("--actions", "'()"),
            ("(action:send-wait 100) --actions", "'((wait 100))"),
            ("(action:send-wait 200) --actions", "'((wait 200) (wait 100))"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(action:send-wait 1.1)",
            "(action:send-wait #t)",
            "(action:send-wait #f)",
            "(action:send-wait \"string\")",
            "(action:send-wait 'symbol)",
            "(action:send-wait :keyword)",
            "(action:send-wait '(s-expression))",
            "(action:send-wait {})",
            "(action:send-wait #())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(action:send-wait)",
            "(action:send-wait 3 \"at\")"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
