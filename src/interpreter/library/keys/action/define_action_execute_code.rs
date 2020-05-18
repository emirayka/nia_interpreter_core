use crate::Error;
use crate::Interpreter;

use crate::library;

pub fn define_action_execute_code<S>(
    interpreter: &mut Interpreter,
    action_name: S,
    code_to_execute: S,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let action_name = action_name.as_ref();
    let code_to_execute = code_to_execute.as_ref();

    let action_execute_code_string_value =
        interpreter.intern_string_value("execute-code");
    let code_to_execute_value =
        interpreter.intern_string_value(code_to_execute);

    let action_value = interpreter.vec_to_list(vec![
        action_execute_code_string_value,
        code_to_execute_value,
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
    fn adds_execute_code_actions_to_action_alist() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                "print-eh",
                r#"(println "eh")"#,
                r#"(list:new (cons:new "print-eh" (list:new "execute-code" "(println \"eh\")")))"#,
            ),
            (
                "print-nya",
                r#"(println "nya")"#,
                r#"(list:new (cons:new "print-nya" (list:new "execute-code" "(println \"nya\")")) (cons:new "print-eh" (list:new "execute-code" "(println \"eh\")")))"#,
            ),
            (
                "print-nia",
                r#"(println "nia")"#,
                r#"(list:new (cons:new "print-nia" (list:new "execute-code" "(println \"nia\")")) (cons:new "print-nya" (list:new "execute-code" "(println \"nya\")")) (cons:new "print-eh" (list:new "execute-code" "(println \"eh\")")))"#,
            ),
        ];

        for (action_name, action_code, expected) in specs {
            let expected =
                interpreter.execute_in_root_environment(expected).unwrap();

            nia_assert_is_ok(&define_action_execute_code(
                &mut interpreter,
                action_name,
                action_code,
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

        nia_assert_is_ok(&define_action_execute_code(
            &mut interpreter,
            "print-kek",
            "(println \"kek\")",
        ));

        crate::utils::assert_generic_execution_error(
            &define_action_execute_code(
                &mut interpreter,
                "print-kek",
                "(println \"kek\")",
            ),
        );
    }
}
