use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn send_mouse_button_down(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `action:send-mouse-button-down' takes one argument exactly."
        ).into();
    }

    let mut values = values;

    let key_code = match values.remove(0) {
        Value::Integer(key_code) => key_code,
        _ => return Error::invalid_argument_error(
            "Built-in function `action:send-mouse-button-down' takes only an integer."
        ).into()
    };

    let mouse_button_down_symbol_value = interpreter.intern_symbol_value("mouse-button-down");
    let mouse_button_down = interpreter.vec_to_list(vec!(
        mouse_button_down_symbol_value,
        Value::Integer(key_code)
    ));

    library::add_value_to_root_list(
        interpreter,
        "--actions",
        mouse_button_down
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
            ("(action:send-mouse-button-down 2) --actions", "'((mouse-button-down 2))"),
            ("(action:send-mouse-button-down 3) --actions", "'((mouse-button-down 3) (mouse-button-down 2))"),
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
            "(action:send-mouse-button-down 1.1)",
            "(action:send-mouse-button-down #t)",
            "(action:send-mouse-button-down #f)",
            "(action:send-mouse-button-down \"string\")",
            "(action:send-mouse-button-down 'symbol)",
            "(action:send-mouse-button-down :keyword)",
            "(action:send-mouse-button-down '(s-expression))",
            "(action:send-mouse-button-down {})",
            "(action:send-mouse-button-down #())",
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
            "(action:send-mouse-button-down)",
            "(action:send-mouse-button-down 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
