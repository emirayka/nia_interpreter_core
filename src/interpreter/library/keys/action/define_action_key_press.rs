use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn define_action_key_press<S>(
    interpreter: &mut Interpreter,
    action_name: S,
    key_code: i32,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let action_name = action_name.as_ref();

    let action_key_press_string_value =
        interpreter.intern_string_value("key-press");
    let key_code_value = Value::Integer(key_code as i64);

    let action_value = interpreter
        .vec_to_list(vec![action_key_press_string_value, key_code_value]);

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
    fn adds_key_press_actions_to_action_alist() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                "press-q",
                16,
                r#"(list:new (cons:new "press-q" (list:new "key-press" 16)))"#,
            ),
            (
                "press-w",
                17,
                r#"(list:new (cons:new "press-w" (list:new "key-press" 17)) (cons:new "press-q" (list:new "key-press" 16)))"#,
            ),
            (
                "press-f",
                33,
                r#"(list:new (cons:new "press-f" (list:new "key-press" 33)) (cons:new "press-w" (list:new "key-press" 17)) (cons:new "press-q" (list:new "key-press" 16)))"#,
            ),
        ];

        for (action_name, key_code, expected) in specs {
            let expected =
                interpreter.execute_in_root_environment(expected).unwrap();

            nia_assert_is_ok(&define_action_key_press(
                &mut interpreter,
                action_name,
                key_code,
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

        nia_assert_is_ok(&define_action_key_press(
            &mut interpreter,
            "press-q",
            16,
        ));

        crate::utils::assert_generic_execution_error(&define_action_key_press(
            &mut interpreter,
            "press-q",
            16,
        ));
    }
}
