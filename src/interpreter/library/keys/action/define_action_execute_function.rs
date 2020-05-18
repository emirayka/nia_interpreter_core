use crate::Error;
use crate::Interpreter;

use crate::library;

pub fn define_action_execute_function<S>(
    interpreter: &mut Interpreter,
    action_name: S,
    function_name: S,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let action_name = action_name.as_ref();
    let action_function_name = function_name.as_ref();

    let action_type_execute_function_value =
        interpreter.intern_string_value("execute-function");
    let action_function_name_value =
        interpreter.intern_string_value(action_function_name);

    let action_value = interpreter.vec_to_list(vec![
        action_type_execute_function_value,
        action_function_name_value,
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
    fn adds_execute_function_actions_to_action_alist() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                "execute-eh",
                r#"eh"#,
                r#"(list:new (cons:new "execute-eh" (list:new "execute-function" "eh")))"#,
            ),
            (
                "execute-nya",
                r#"nya"#,
                r#"(list:new (cons:new "execute-nya" (list:new "execute-function" "nya")) (cons:new "execute-eh" (list:new "execute-function" "eh")))"#,
            ),
            (
                "execute-nia",
                r#"nia"#,
                r#"(list:new (cons:new "execute-nia" (list:new "execute-function" "nia")) (cons:new "execute-nya" (list:new "execute-function" "nya")) (cons:new "execute-eh" (list:new "execute-function" "eh")))"#,
            ),
        ];

        for (action_name, action_function, expected) in specs {
            let expected =
                interpreter.execute_in_root_environment(expected).unwrap();

            nia_assert_is_ok(&define_action_execute_function(
                &mut interpreter,
                action_name,
                action_function,
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

        nia_assert_is_ok(&define_action_execute_function(
            &mut interpreter,
            "execute-kek",
            "kek",
        ));

        crate::utils::assert_generic_execution_error(
            &define_action_execute_function(
                &mut interpreter,
                "execute-kek",
                "kek",
            ),
        );
    }
}
