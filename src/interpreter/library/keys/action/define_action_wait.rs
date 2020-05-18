use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn define_action_wait<S>(
    interpreter: &mut Interpreter,
    action_name: S,
    ms_amount: i32,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let action_name = action_name.as_ref();

    let action_type_wait_string_value = interpreter.intern_string_value("wait");
    let ms_amount_value = Value::Integer(ms_amount as i64);

    let action_value = interpreter
        .vec_to_list(vec![action_type_wait_string_value, ms_amount_value]);

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
    fn adds_execute_amount_actions_to_action_alist() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                "wait-1-sec",
                1000,
                r#"(list:new (cons:new "wait-1-sec" (list:new "wait" 1000)))"#,
            ),
            (
                "wait-2-sec",
                2000,
                r#"(list:new (cons:new "wait-2-sec" (list:new "wait" 2000)) (cons:new "wait-1-sec" (list:new "wait" 1000)))"#,
            ),
            (
                "wait-3-sec",
                3000,
                r#"(list:new (cons:new "wait-3-sec" (list:new "wait" 3000)) (cons:new "wait-2-sec" (list:new "wait" 2000)) (cons:new "wait-1-sec" (list:new "wait" 1000)))"#,
            ),
        ];

        for (action_name, ms_amount, expected) in specs {
            let expected =
                interpreter.execute_in_root_environment(expected).unwrap();

            nia_assert_is_ok(&define_action_wait(
                &mut interpreter,
                action_name,
                ms_amount,
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

        nia_assert_is_ok(&define_action_wait(
            &mut interpreter,
            "wait-1-sec",
            1000,
        ));

        crate::utils::assert_generic_execution_error(&define_action_wait(
            &mut interpreter,
            "wait-1-sec",
            1000,
        ));
    }
}
