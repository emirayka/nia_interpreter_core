use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn send_key_press(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `action:send-key-press' takes one argument exactly."
        ).into_result();
    }

    let mut values = values;

    let key_code = match values.remove(0) {
        Value::Integer(key_code) => key_code,
        Value::String(key_name) => {
            let key_name = interpreter.get_string(key_name)?;

            let key_id = nia_events::str_to_key_id(key_name.get_string())
                .map_err(|_| Error::invalid_argument_error(
                    &format!("Invalid key name: {}", key_name.get_string())
                ))?.get_id() as i64;

            key_id
        },
        _ => return Error::invalid_argument_error(
            "Built-in function `action:send-key-press' takes only an integer or string."
        ).into_result()
    };

    let key_press_symbol_value = interpreter.intern_symbol_value("key-press");
    let key_press = interpreter.vec_to_list(vec!(
        key_press_symbol_value,
        Value::Integer(key_code)
    ));

    library::add_value_to_root_list(
        interpreter,
        "--actions",
        key_press
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
            ("(action:send-key-press 2) --actions", "'((key-press 2))"),
            ("(action:send-key-press 3) --actions", "'((key-press 3) (key-press 2))"),
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
            "(action:send-key-press 1.1)",
            "(action:send-key-press #t)",
            "(action:send-key-press #f)",
            "(action:send-key-press 'symbol)",
            "(action:send-key-press :keyword)",
            "(action:send-key-press '(s-expression))",
            "(action:send-key-press {})",
            "(action:send-key-press #())",
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
            "(action:send-key-press)",
            "(action:send-key-press 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
