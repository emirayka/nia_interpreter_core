use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn send_mouse_button_press(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `action:send-mouse-button-press' takes one argument exactly."
        ).into();
    }

    let mut values = values;

    let key_code = match values.remove(0) {
        Value::Integer(key_code) => key_code,
        _ => return Error::invalid_argument_error(
            "Built-in function `action:send-mouse-button-press' takes only an integer."
        ).into()
    };

    let mouse_button_press_symbol_value = interpreter.intern_symbol_value("mouse-button-press");
    let mouse_button_press = interpreter.vec_to_list(vec!(
        mouse_button_press_symbol_value,
        Value::Integer(key_code)
    ));

    library::add_value_to_root_list(
        interpreter,
        "--actions",
        mouse_button_press
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
            ("(action:send-mouse-button-press 2) --actions", "'((mouse-button-press 2))"),
            ("(action:send-mouse-button-press 3) --actions", "'((mouse-button-press 3) (mouse-button-press 2))"),
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
            "(action:send-mouse-button-press 1.1)",
            "(action:send-mouse-button-press #t)",
            "(action:send-mouse-button-press #f)",
            "(action:send-mouse-button-press \"string\")",
            "(action:send-mouse-button-press 'symbol)",
            "(action:send-mouse-button-press :keyword)",
            "(action:send-mouse-button-press '(s-expression))",
            "(action:send-mouse-button-press {})",
            "(action:send-mouse-button-press #())",
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
            "(action:send-mouse-button-press)",
            "(action:send-mouse-button-press 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
