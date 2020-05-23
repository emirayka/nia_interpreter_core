use nia_events::KeyId;

use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::PRIMITIVE_ACTIONS_VARIABLE_NAME;

use crate::interpreter::library;

pub fn send_key_click(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `action:send-key-click' takes one argument exactly.",
        )
        .into();
    }

    let mut values = values;

    let key_code = match values.remove(0) {
        Value::Integer(key_code) => key_code,
        _ => return Error::invalid_argument_error(
            "Built-in function `action:send-key-click' takes only an integer.",
        )
        .into(),
    };

    let key_press_symbol_value = interpreter.intern_symbol_value("key-click");
    let key_press = interpreter
        .vec_to_list(vec![key_press_symbol_value, Value::Integer(key_code)]);

    library::add_value_to_root_list(
        interpreter,
        PRIMITIVE_ACTIONS_VARIABLE_NAME,
        key_press,
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
                "(action:send-key-click 2) nia-primitive-actions",
                "'((key-click 2))",
            ),
            (
                "(action:send-key-click 3) nia-primitive-actions",
                "'((key-click 3) (key-click 2))",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(action:send-key-click 1.1)",
            "(action:send-key-click #t)",
            "(action:send-key-click #f)",
            "(action:send-key-click 'symbol)",
            "(action:send-key-click :keyword)",
            "(action:send-key-click '(s-expression))",
            "(action:send-key-click {})",
            "(action:send-key-click #())",
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

        let code_vector =
            vec!["(action:send-key-click)", "(action:send-key-click 1 2)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
