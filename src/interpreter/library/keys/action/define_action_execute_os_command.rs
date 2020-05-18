use crate::Error;
use crate::Interpreter;

use crate::library;

pub fn define_action_execute_os_command<S>(
    interpreter: &mut Interpreter,
    action_name: S,
    os_command: S,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let action_name = action_name.as_ref();
    let action_os_command = os_command.as_ref();

    let action_type_execute_os_command_value =
        interpreter.intern_string_value("execute-os-command");
    let action_os_command_value =
        interpreter.intern_string_value(action_os_command);

    let action_value = interpreter.vec_to_list(vec![
        action_type_execute_os_command_value,
        action_os_command_value,
    ]);

    library::define_action(interpreter, action_name, action_value)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::DEFINED_ACTIONS_ROOT_VARIABLE_NAME;

    #[test]
    fn adds_execute_os_command_actions_to_action_alist() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                "execute-eh",
                r#"eh"#,
                r#"(list:new (cons:new "execute-eh" (list:new "execute-os-command" "eh")))"#,
            ),
            (
                "execute-nya",
                r#"nya"#,
                r#"(list:new (cons:new "execute-nya" (list:new "execute-os-command" "nya")) (cons:new "execute-eh" (list:new "execute-os-command" "eh")))"#,
            ),
            (
                "execute-nia",
                r#"nia"#,
                r#"(list:new (cons:new "execute-nia" (list:new "execute-os-command" "nia")) (cons:new "execute-nya" (list:new "execute-os-command" "nya")) (cons:new "execute-eh" (list:new "execute-os-command" "eh")))"#,
            ),
        ];

        for (action_name, action_os_command, expected) in specs {
            let expected =
                interpreter.execute_in_root_environment(expected).unwrap();

            nia_assert_is_ok(&define_action_execute_os_command(
                &mut interpreter,
                action_name,
                action_os_command,
            ));

            let result = library::get_root_variable(
                &mut interpreter,
                DEFINED_ACTIONS_ROOT_VARIABLE_NAME,
            )
            .unwrap();

            crate::utils::assert_deep_equal(&mut interpreter, expected, result);
        }
    }

    #[test]
    fn returns_error_when_action_with_that_name_already_defined() {
        let mut interpreter = Interpreter::new();

        nia_assert_is_ok(&define_action_execute_os_command(
            &mut interpreter,
            "execute-kek",
            "kek",
        ));

        crate::utils::assert_generic_execution_error(
            &define_action_execute_os_command(
                &mut interpreter,
                "execute-kek",
                "kek",
            ),
        );
    }
}
