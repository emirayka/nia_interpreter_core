use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;
use crate::PRIMITIVE_ACTIONS_VARIABLE_NAME;

pub fn send_mouse_button_release(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `action:send-mouse-button-release' takes one argument exactly.",
        )
        .into();
    }

    let mut values = values;

    let key_code = match values.remove(0) {
        Value::Integer(key_code) => key_code,
        _ => {
            return Error::invalid_argument_error(
                "Built-in function `action:send-mouse-button-release' takes only an integer.",
            )
            .into()
        }
    };

    let mouse_button_release_symbol_value =
        interpreter.intern_symbol_value("mouse-button-release");
    let mouse_button_release = interpreter.vec_to_list(vec![
        mouse_button_release_symbol_value,
        Value::Integer(key_code),
    ]);

    library::add_value_to_root_list(
        interpreter,
        PRIMITIVE_ACTIONS_VARIABLE_NAME,
        mouse_button_release,
    )?;

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn adds_action_to_action_list() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (PRIMITIVE_ACTIONS_VARIABLE_NAME, "'()"),
            (
                "(action:send-mouse-button-release 2) nia-primitive-actions",
                "'((mouse-button-release 2))",
            ),
            (
                "(action:send-mouse-button-release 3) nia-primitive-actions",
                "'((mouse-button-release 3) (mouse-button-release 2))",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(action:send-mouse-button-release 1.1)",
            "(action:send-mouse-button-release #t)",
            "(action:send-mouse-button-release #f)",
            "(action:send-mouse-button-release \"string\")",
            "(action:send-mouse-button-release 'symbol)",
            "(action:send-mouse-button-release :keyword)",
            "(action:send-mouse-button-release '(s-expression))",
            "(action:send-mouse-button-release {})",
            "(action:send-mouse-button-release #())",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(action:send-mouse-button-release)",
            "(action:send-mouse-button-release 1 2)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
