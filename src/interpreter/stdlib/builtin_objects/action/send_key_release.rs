use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;
use crate::PRIMITIVE_ACTIONS_VARIABLE_NAME;

pub fn send_key_release(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `action:send-key-release' takes one argument exactly.",
        )
        .into();
    }

    let mut values = values;

    let key_code = match values.remove(0) {
        Value::Integer(key_code) => key_code,
        _ => {
            return Error::invalid_argument_error(
                "Built-in function `action:send-key-release' takes only an integer or string.",
            )
            .into()
        }
    };

    let key_up_symbol_value = interpreter.intern_symbol_value("key-release");
    let key_up = interpreter
        .vec_to_list(vec![key_up_symbol_value, Value::Integer(key_code)]);

    library::add_value_to_root_list(
        interpreter,
        PRIMITIVE_ACTIONS_VARIABLE_NAME,
        key_up,
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
                "(action:send-key-release 2) nia-primitive-actions",
                "'((key-release 2))",
            ),
            (
                "(action:send-key-release 3) nia-primitive-actions",
                "'((key-release 3) (key-release 2))",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(action:send-key-release 1.1)",
            "(action:send-key-release #t)",
            "(action:send-key-release #f)",
            "(action:send-key-release 'symbol)",
            "(action:send-key-release :keyword)",
            "(action:send-key-release '(s-expression))",
            "(action:send-key-release {})",
            "(action:send-key-release #())",
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
            vec!["(action:send-key-release)", "(action:send-key-release 1 2)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
