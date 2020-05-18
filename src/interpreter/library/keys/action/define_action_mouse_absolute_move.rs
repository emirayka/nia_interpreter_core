use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn define_action_mouse_absolute_move<S>(
    interpreter: &mut Interpreter,
    action_name: S,
    x: i32,
    y: i32,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let action_name = action_name.as_ref();

    let action_mouse_absolute_move_string_value =
        interpreter.intern_string_value("mouse-absolute-move");
    let x_value = Value::Integer(x as i64);
    let y_value = Value::Integer(y as i64);

    let action_value = interpreter.vec_to_list(vec![
        action_mouse_absolute_move_string_value,
        x_value,
        y_value,
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
    fn adds_mouse_absolute_move_actions_to_action_alist() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                "mouse-absolute-move-1",
                100,
                100,
                r#"(list:new (cons:new "mouse-absolute-move-1" (list:new "mouse-absolute-move" 100 100)))"#,
            ),
            (
                "mouse-absolute-move-2",
                200,
                200,
                r#"(list:new (cons:new "mouse-absolute-move-2" (list:new "mouse-absolute-move" 200 200)) (cons:new "mouse-absolute-move-1" (list:new "mouse-absolute-move" 100 100)))"#,
            ),
            (
                "mouse-absolute-move-3",
                300,
                300,
                r#"(list:new (cons:new "mouse-absolute-move-3" (list:new "mouse-absolute-move" 300 300)) (cons:new "mouse-absolute-move-2" (list:new "mouse-absolute-move" 200 200)) (cons:new "mouse-absolute-move-1" (list:new "mouse-absolute-move" 100 100)))"#,
            ),
        ];

        for (action_name, x, y, expected) in specs {
            let expected =
                interpreter.execute_in_root_environment(expected).unwrap();

            nia_assert_is_ok(&define_action_mouse_absolute_move(
                &mut interpreter,
                action_name,
                x,
                y,
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

        nia_assert_is_ok(&define_action_mouse_absolute_move(
            &mut interpreter,
            "mouse-absolute-move-1",
            100,
            100,
        ));

        crate::utils::assert_generic_execution_error(
            &define_action_mouse_absolute_move(
                &mut interpreter,
                "mouse-absolute-move-1",
                100,
                100,
            ),
        );
    }
}
