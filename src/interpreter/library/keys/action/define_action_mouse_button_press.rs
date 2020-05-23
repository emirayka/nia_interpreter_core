use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn define_action_mouse_button_press<S>(
    interpreter: &mut Interpreter,
    action_name: S,
    button_code: i32,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let action_name = action_name.as_ref();

    let action_mouse_button_press_value =
        interpreter.intern_symbol_value("mouse-button-press");
    let button_code_value = Value::Integer(button_code as i64);

    let action_value = interpreter
        .vec_to_list(vec![action_mouse_button_press_value, button_code_value]);

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
    fn adds_mouse_button_press_actions_to_action_alist() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                "mouse-button-press-left",
                1,
                r#"(list:new (cons:new "mouse-button-press-left" (list:new 'mouse-button-press 1)))"#,
            ),
            (
                "mouse-button-press-right",
                2,
                r#"(list:new (cons:new "mouse-button-press-right" (list:new 'mouse-button-press 2)) (cons:new "mouse-button-press-left" (list:new 'mouse-button-press 1)))"#,
            ),
            (
                "mouse-button-press-middle",
                3,
                r#"(list:new (cons:new "mouse-button-press-middle" (list:new 'mouse-button-press 3)) (cons:new "mouse-button-press-right" (list:new 'mouse-button-press 2)) (cons:new "mouse-button-press-left" (list:new 'mouse-button-press 1)))"#,
            ),
        ];

        for (action_name, mouse_mouse_button_code, expected) in specs {
            let expected =
                interpreter.execute_in_root_environment(expected).unwrap();

            nia_assert_is_ok(&define_action_mouse_button_press(
                &mut interpreter,
                action_name,
                mouse_mouse_button_code,
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

        nia_assert_is_ok(&define_action_mouse_button_press(
            &mut interpreter,
            "mouse-button-press-left",
            1,
        ));

        crate::utils::assert_generic_execution_error(
            &define_action_mouse_button_press(
                &mut interpreter,
                "mouse-button-press-left",
                1,
            ),
        );
    }
}
