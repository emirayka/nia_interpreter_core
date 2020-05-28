use crate::{Error, DEFINED_ACTIONS_ROOT_VARIABLE_NAME};
use crate::{Interpreter, Value};

use crate::library;

pub fn define_action_with_value<S>(
    interpreter: &mut Interpreter,
    action_name: S,
    action_value: Value,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let action_name = action_name.as_ref();
    let action_name_string_value = interpreter.intern_string_value(action_name);

    if library::is_root_alist_has_key(
        interpreter,
        action_name_string_value,
        DEFINED_ACTIONS_ROOT_VARIABLE_NAME,
    )? {
        return Error::generic_execution_error(format!(
            "Action {} already defined.",
            action_name
        ))
        .into();
    }

    library::add_item_to_root_alist(
        interpreter,
        action_name_string_value,
        action_value,
        DEFINED_ACTIONS_ROOT_VARIABLE_NAME,
    )
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
                interpreter.intern_symbol_value("do-nothing"),
                r#"(list:new (cons:new "stub-1" 'do-nothing))"#,
            ),
            (
                "stub-2",
                interpreter.intern_symbol_value("do-nothing-2"),
                r#"(list:new (cons:new "stub-2" 'do-nothing-2) (cons:new "stub-1" 'do-nothing))"#,
            ),
            (
                "stub-3",
                interpreter.intern_symbol_value("do-nothing-3"),
                r#"(list:new (cons:new "stub-3" 'do-nothing-3) (cons:new "stub-2" 'do-nothing-2) (cons:new "stub-1" 'do-nothing))"#,
            ),
        ];

        for (action_name, action_value, expected) in specs {
            let expected =
                interpreter.execute_in_root_environment(expected).unwrap();

            nia_assert_is_ok(&define_action_with_value(
                &mut interpreter,
                action_name,
                action_value,
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

        nia_assert_is_ok(&define_action_with_value(
            &mut interpreter,
            "print-kek",
            Value::Integer(1),
        ));

        crate::utils::assert_generic_execution_error(
            &define_action_with_value(
                &mut interpreter,
                "print-kek",
                Value::Integer(1),
            ),
        );
    }
}
