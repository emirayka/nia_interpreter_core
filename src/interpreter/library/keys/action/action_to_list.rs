use crate::Action;
use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::DEFINED_ACTIONS_ROOT_VARIABLE_NAME;

use crate::library;

pub fn action_to_list(
    interpreter: &mut Interpreter,
    action: &Action,
) -> Result<Value, Error> {
    let action_value_vector = match action {
        &Action::KeyPress(key_code) => vec![
            interpreter.intern_symbol_value("key-press"),
            Value::Integer(key_code as i64),
        ],
        &Action::KeyClick(key_code) => vec![
            interpreter.intern_symbol_value("key-click"),
            Value::Integer(key_code as i64),
        ],
        &Action::KeyRelease(key_code) => vec![
            interpreter.intern_symbol_value("key-release"),
            Value::Integer(key_code as i64),
        ],
        &Action::MouseButtonPress(mouse_button_code) => vec![
            interpreter.intern_symbol_value("mouse-button-press"),
            Value::Integer(mouse_button_code as i64),
        ],
        &Action::MouseButtonClick(mouse_button_code) => vec![
            interpreter.intern_symbol_value("mouse-button-click"),
            Value::Integer(mouse_button_code as i64),
        ],
        &Action::MouseButtonRelease(mouse_button_code) => vec![
            interpreter.intern_symbol_value("mouse-button-release"),
            Value::Integer(mouse_button_code as i64),
        ],
        &Action::MouseAbsoluteMove(x, y) => vec![
            interpreter.intern_symbol_value("mouse-absolute-move"),
            Value::Integer(x as i64),
            Value::Integer(y as i64),
        ],
        &Action::MouseRelativeMove(dx, dy) => vec![
            interpreter.intern_symbol_value("mouse-relative-move"),
            Value::Integer(dx as i64),
            Value::Integer(dy as i64),
        ],
        Action::TextType(text_to_type) => vec![
            interpreter.intern_symbol_value("text-type"),
            interpreter.intern_string_value(text_to_type),
        ],
        Action::ExecuteCode(code) => vec![
            interpreter.intern_symbol_value("execute-code"),
            interpreter.intern_string_value(code),
        ],
        Action::ExecuteFunction(function_name) => vec![
            interpreter.intern_symbol_value("execute-function"),
            interpreter.intern_string_value(function_name),
        ],
        Action::ExecuteOSCommand(os_command) => vec![
            interpreter.intern_symbol_value("execute-os-command"),
            interpreter.intern_string_value(os_command),
        ],
        &Action::Wait(ms_amount) => vec![
            interpreter.intern_symbol_value("wait"),
            Value::Integer(ms_amount as i64),
        ],
        &Action::ExecuteFunctionValue(function_value) => vec![
            interpreter.intern_symbol_value("execute-function-value"),
            function_value,
        ],
    };

    let action_list = interpreter.vec_to_list(action_value_vector);

    Ok(action_list)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::utils;

    #[test]
    fn correctly_converts_actions() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (r#"'(key-press 1)"#, Action::KeyPress(1)),
            (r#"'(key-click 1)"#, Action::KeyClick(1)),
            (r#"'(key-release 1)"#, Action::KeyRelease(1)),
            (r#"'(mouse-button-press 1)"#, Action::MouseButtonPress(1)),
            (r#"'(mouse-button-click 1)"#, Action::MouseButtonClick(1)),
            (
                r#"'(mouse-button-release 1)"#,
                Action::MouseButtonRelease(1),
            ),
            (
                r#"'(mouse-absolute-move 100 100)"#,
                Action::MouseAbsoluteMove(100, 100),
            ),
            (
                r#"'(mouse-relative-move 100 100)"#,
                Action::MouseRelativeMove(100, 100),
            ),
            (
                r#"'(text-type "text")"#,
                Action::TextType(String::from("text")),
            ),
            (
                r#"'(execute-os-command "os-command")"#,
                Action::ExecuteOSCommand(String::from("os-command")),
            ),
            (
                r#"'(execute-code "code snippet")"#,
                Action::ExecuteCode(String::from("code snippet")),
            ),
            (
                r#"'(execute-function "function name")"#,
                Action::ExecuteFunction(String::from("function name")),
            ),
            (r#"'(wait 1000)"#, Action::Wait(1000)),
        ];

        for (expected, action) in specs {
            let expected =
                interpreter.execute_in_main_environment(expected).unwrap();
            let result = action_to_list(&mut interpreter, &action).unwrap();

            utils::assert_deep_equal(&mut interpreter, expected, result);
        }
    }

    #[test]
    fn correctly_converts_execute_function_value() {
        let mut interpreter = Interpreter::new();

        let execute_function_value_symbol_id =
            interpreter.intern_symbol_value("execute-function-value");
        let function_value =
            interpreter.execute_in_main_environment("#()").unwrap();

        let expected = interpreter.vec_to_list(vec![
            execute_function_value_symbol_id,
            function_value,
        ]);
        let result = action_to_list(
            &mut interpreter,
            &Action::ExecuteFunctionValue(function_value),
        )
        .unwrap();

        utils::assert_deep_equal(&mut interpreter, expected, result);
    }
}
