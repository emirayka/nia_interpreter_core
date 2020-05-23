use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;
use crate::PRIMITIVE_ACTIONS_VARIABLE_NAME;

pub fn send_execute_os_command(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `action:send-execute-os-command' takes one argument exactly.",
        )
        .into();
    }

    let mut values = values;

    let text = library::read_as_string_id(values.remove(0))?;

    let execute_os_command_symbol_value =
        interpreter.intern_symbol_value("execute-os-command");
    let execute_os_command = interpreter.vec_to_list(vec![
        execute_os_command_symbol_value,
        Value::String(text),
    ]);

    library::add_value_to_root_list(
        interpreter,
        PRIMITIVE_ACTIONS_VARIABLE_NAME,
        execute_os_command,
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
                "(action:send-execute-os-command \"first\") nia-primitive-actions",
                "'((execute-os-command \"first\"))",
            ),
            (
                "(action:send-execute-os-command \"second\") nia-primitive-actions",
                "'((execute-os-command \"second\") (execute-os-command \"first\"))",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(action:send-execute-os-command 1)",
            "(action:send-execute-os-command 1.1)",
            "(action:send-execute-os-command #t)",
            "(action:send-execute-os-command #f)",
            "(action:send-execute-os-command 'symbol)",
            "(action:send-execute-os-command :keyword)",
            "(action:send-execute-os-command '(s-expression))",
            "(action:send-execute-os-command {})",
            "(action:send-execute-os-command #())",
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
            "(action:send-execute-os-command)",
            "(action:send-execute-os-command \"at\" 3)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
