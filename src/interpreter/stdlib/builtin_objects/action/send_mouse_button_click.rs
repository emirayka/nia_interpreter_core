use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;
use crate::PRIMITIVE_ACTIONS_VARIABLE_NAME;

pub fn send_mouse_button_click(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `action:send-mouse-button-click' takes one argument exactly.",
        )
        .into();
    }

    let mut values = values;

    let key_code = match values.remove(0) {
        Value::Integer(key_code) => key_code,
        _ => {
            return Error::invalid_argument_error(
                "Built-in function `action:send-mouse-button-click' takes only an integer.",
            )
            .into()
        }
    };

    let mouse_button_click_symbol_value =
        interpreter.intern_symbol_value("mouse-button-click");
    let mouse_button_click = interpreter.vec_to_list(vec![
        mouse_button_click_symbol_value,
        Value::Integer(key_code),
    ]);

    library::add_value_to_root_list(
        interpreter,
        PRIMITIVE_ACTIONS_VARIABLE_NAME,
        mouse_button_click,
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
                "(action:send-mouse-button-click 2) nia-primitive-actions",
                "'((mouse-button-click 2))",
            ),
            (
                "(action:send-mouse-button-click 3) nia-primitive-actions",
                "'((mouse-button-click 3) (mouse-button-click 2))",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(action:send-mouse-button-click 1.1)",
            "(action:send-mouse-button-click #t)",
            "(action:send-mouse-button-click #f)",
            "(action:send-mouse-button-click \"string\")",
            "(action:send-mouse-button-click 'symbol)",
            "(action:send-mouse-button-click :keyword)",
            "(action:send-mouse-button-click '(s-expression))",
            "(action:send-mouse-button-click {})",
            "(action:send-mouse-button-click #())",
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
            "(action:send-mouse-button-click)",
            "(action:send-mouse-button-click 1 2)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
