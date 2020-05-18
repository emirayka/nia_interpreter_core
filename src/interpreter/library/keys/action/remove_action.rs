use crate::Error;
use crate::Interpreter;
use crate::DEFINED_ACTIONS_ROOT_VARIABLE_NAME;

use crate::library;

pub fn remove_action<S>(
    interpreter: &mut Interpreter,
    action_name: S,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let action_name = action_name.as_ref();
    let action_name_value = interpreter.intern_string_value(action_name);

    if !library::is_action_defined(interpreter, action_name)? {
        return Error::generic_execution_error(format!(
            "Action {} is not found",
            action_name
        ))
        .into();
    }

    library::remove_item_from_root_alist(
        interpreter,
        DEFINED_ACTIONS_ROOT_VARIABLE_NAME,
        action_name_value,
    )
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn removes_defined_action() {
        let mut interpreter = Interpreter::new();

        library::define_action_wait(&mut interpreter, "wait-1-sec", 1000)
            .unwrap();

        library::define_action_wait(&mut interpreter, "wait-2-sec", 2000)
            .unwrap();

        library::define_action_wait(&mut interpreter, "wait-3-sec", 3000)
            .unwrap();

        let specs = vec![
            (
                "wait-1-sec",
                r#"(list:new (cons:new "wait-3-sec" (list:new "wait" 3000)) (cons:new "wait-2-sec" (list:new "wait" 2000)))"#,
            ),
            (
                "wait-3-sec",
                r#"(list:new (cons:new "wait-2-sec" (list:new "wait" 2000)))"#,
            ),
            ("wait-2-sec", r#"(list:new)"#),
        ];

        for (action_name, expected) in specs {
            let expected =
                interpreter.execute_in_root_environment(expected).unwrap();

            nia_assert_is_ok(&remove_action(&mut interpreter, action_name));

            let result = library::get_root_variable(
                &mut interpreter,
                DEFINED_ACTIONS_ROOT_VARIABLE_NAME,
            )
            .unwrap();

            crate::utils::assert_deep_equal(&mut interpreter, expected, result);
        }
    }

    #[test]
    fn returns_error_when_no_action_with_such_name_were_defined() {
        let mut interpreter = Interpreter::new();

        library::define_action_wait(&mut interpreter, "wait-1-sec", 1000)
            .unwrap();

        &library::remove_action(&mut interpreter, "wait-1-sec");

        crate::utils::assert_generic_execution_error(&library::remove_action(
            &mut interpreter,
            "wait-1-sec",
        ));
    }
}
