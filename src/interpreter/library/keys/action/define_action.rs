use crate::{Action, Error, DEFINED_ACTIONS_ROOT_VARIABLE_NAME};
use crate::{Interpreter, Value};

use crate::library;

pub fn define_action<S>(
    interpreter: &mut Interpreter,
    action_name: S,
    action: &Action,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let action_name = action_name.as_ref();
    let action_value = library::action_to_list(interpreter, action)?;

    library::define_action_with_value(interpreter, action_name, action_value)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn adds_execute_code_actions_to_action_alist() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                "stub-1",
                Action::KeyClick(1),
                r#"(list:new (list:new "stub-1" 'key-click 1))"#,
            ),
            (
                "stub-2",
                Action::KeyClick(2),
                r#"(list:new (list:new "stub-2" 'key-click 2) (list:new "stub-1" 'key-click 1))"#,
            ),
            (
                "stub-3",
                Action::KeyClick(3),
                r#"(list:new (list:new "stub-3" 'key-click 3) (list:new "stub-2" 'key-click 2) (list:new "stub-1" 'key-click 1))"#,
            ),
        ];

        for (action_name, action, expected) in specs {
            let expected =
                interpreter.execute_in_root_environment(expected).unwrap();

            nia_assert_is_ok(&define_action(
                &mut interpreter,
                action_name,
                &action,
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

        nia_assert_is_ok(&define_action(
            &mut interpreter,
            "print-kek",
            &Action::Wait(1000),
        ));

        crate::utils::assert_generic_execution_error(&define_action(
            &mut interpreter,
            "print-kek",
            &Action::Wait(1000),
        ));
    }
}
