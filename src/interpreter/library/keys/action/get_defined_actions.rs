use crate::Action;
use crate::Error;
use crate::Interpreter;
use crate::Value;
use crate::DEFINED_ACTIONS_ROOT_VARIABLE_NAME;

use crate::library;

fn read_string_from_vector(
    interpreter: &mut Interpreter,
    values: &mut Vec<Value>,
) -> Result<String, Error> {
    if values.len() == 0 {
        return Error::generic_execution_error(
            "Action vector has invalid length.",
        )
        .into();
    }

    let string =
        library::read_as_string(interpreter, values.remove(0))?.clone();

    Ok(string)
}

fn read_i32_from_vector(values: &mut Vec<Value>) -> Result<i32, Error> {
    if values.len() == 0 {
        return Error::generic_execution_error(
            "Action vector has invalid length.",
        )
        .into();
    }

    let value = library::read_as_i64(values.remove(0))?.clone();

    Ok(value as i32)
}

fn parse_action(
    interpreter: &mut Interpreter,
    action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let mut action_vector = action_vector;

    if action_vector.len() == 0 {
        return Error::generic_execution_error(
            "Invariant violation: action vector must have length > 1",
        )
        .into();
    }

    let action_type_value = action_vector.remove(0);
    let action_type_string =
        library::read_as_string(interpreter, action_type_value)?.clone();

    match action_type_string.as_str() {
        "execute-code" => {
            let code =
                read_string_from_vector(interpreter, &mut action_vector)?;

            Ok(Action::ExecuteCode(code))
        }
        "execute-function" => {
            let function_name =
                read_string_from_vector(interpreter, &mut action_vector)?;

            Ok(Action::ExecuteFunction(function_name))
        }
        "execute-os-command" => {
            let os_command =
                read_string_from_vector(interpreter, &mut action_vector)?;

            Ok(Action::ExecuteOSCommand(os_command))
        }
        "key-click" => {
            let key_code = read_i32_from_vector(&mut action_vector)?;

            Ok(Action::KeyClick(key_code))
        }
        "key-press" => {
            let key_code = read_i32_from_vector(&mut action_vector)?;

            Ok(Action::KeyPress(key_code))
        }
        "key-release" => {
            let key_code = read_i32_from_vector(&mut action_vector)?;

            Ok(Action::KeyRelease(key_code))
        }
        "mouse-button-press" => {
            let key_code = read_i32_from_vector(&mut action_vector)?;

            Ok(Action::MouseButtonPress(key_code))
        }
        "mouse-button-click" => {
            let key_code = read_i32_from_vector(&mut action_vector)?;

            Ok(Action::MouseButtonClick(key_code))
        }
        "mouse-button-release" => {
            let key_code = read_i32_from_vector(&mut action_vector)?;

            Ok(Action::MouseButtonRelease(key_code))
        }
        "mouse-absolute-move" => {
            let x = read_i32_from_vector(&mut action_vector)?;

            let y = read_i32_from_vector(&mut action_vector)?;

            Ok(Action::MouseAbsoluteMove(x, y))
        }
        "mouse-relative-move" => {
            let dx = read_i32_from_vector(&mut action_vector)?;

            let dy = read_i32_from_vector(&mut action_vector)?;

            Ok(Action::MouseRelativeMove(dx, dy))
        }
        "text-type" => {
            let text_to_type =
                read_string_from_vector(interpreter, &mut action_vector)?;

            Ok(Action::TextType(text_to_type))
        }
        "wait" => {
            let ms_amount = read_i32_from_vector(&mut action_vector)?;

            Ok(Action::Wait(ms_amount))
        }
        _ => Error::generic_execution_error(format!(
            "Invalid action type: {}.",
            action_type_string
        ))
        .into(),
    }
}

pub fn get_defined_actions(
    interpreter: &mut Interpreter,
) -> Result<Vec<(String, Action)>, Error> {
    let actions = library::get_root_variable(
        interpreter,
        DEFINED_ACTIONS_ROOT_VARIABLE_NAME,
    )?;

    let action_values = library::read_as_vector(interpreter, actions)?;

    let mut result = Vec::new();

    for action_value in action_values {
        let action_value_cons_id = library::read_as_cons_id(action_value)?;

        let action_name = interpreter.get_car(action_value_cons_id)?;
        let action_name =
            library::read_as_string(interpreter, action_name)?.clone();

        let action_vector = interpreter.get_cdr(action_value_cons_id)?;
        let action_vector =
            library::read_as_vector(interpreter, action_vector)?;

        let action = parse_action(interpreter, action_vector)?;

        result.push((action_name, action));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn returns_defined_actions() {
        let mut interpreter = Interpreter::new();

        library::define_action_execute_code(
            &mut interpreter,
            "execute-code",
            "(println \"lambert, lambert\")",
        )
        .unwrap();

        library::define_action_execute_function(
            &mut interpreter,
            "execute-function",
            "function",
        )
        .unwrap();

        library::define_action_execute_os_command(
            &mut interpreter,
            "execute-os-command",
            "echo \"cat\"",
        )
        .unwrap();

        library::define_action_key_click(&mut interpreter, "key-click", 33)
            .unwrap();

        library::define_action_key_press(&mut interpreter, "key-press", 33)
            .unwrap();

        library::define_action_key_release(&mut interpreter, "key-release", 33)
            .unwrap();

        library::define_action_mouse_button_click(
            &mut interpreter,
            "mouse-button-click",
            1,
        )
        .unwrap();

        library::define_action_mouse_button_press(
            &mut interpreter,
            "mouse-button-press",
            1,
        )
        .unwrap();

        library::define_action_mouse_button_release(
            &mut interpreter,
            "mouse-button-release",
            1,
        )
        .unwrap();

        library::define_action_mouse_absolute_move(
            &mut interpreter,
            "mouse-absolute-move",
            100,
            100,
        )
        .unwrap();

        library::define_action_mouse_relative_move(
            &mut interpreter,
            "mouse-relative-move",
            100,
            100,
        )
        .unwrap();

        library::define_action_text_type(&mut interpreter, "text-type", "cat")
            .unwrap();

        library::define_action_wait(&mut interpreter, "wait", 1000).unwrap();

        let expected = vec![
            (String::from("wait"), Action::Wait(1000)),
            (
                String::from("text-type"),
                Action::TextType(String::from("cat")),
            ),
            (
                String::from("mouse-relative-move"),
                Action::MouseRelativeMove(100, 100),
            ),
            (
                String::from("mouse-absolute-move"),
                Action::MouseAbsoluteMove(100, 100),
            ),
            (
                String::from("mouse-button-release"),
                Action::MouseButtonRelease(1),
            ),
            (
                String::from("mouse-button-press"),
                Action::MouseButtonPress(1),
            ),
            (
                String::from("mouse-button-click"),
                Action::MouseButtonClick(1),
            ),
            (String::from("key-release"), Action::KeyRelease(33)),
            (String::from("key-press"), Action::KeyPress(33)),
            (String::from("key-click"), Action::KeyClick(33)),
            (
                String::from("execute-os-command"),
                Action::ExecuteOSCommand(String::from("echo \"cat\"")),
            ),
            (
                String::from("execute-function"),
                Action::ExecuteFunction(String::from("function")),
            ),
            (
                String::from("execute-code"),
                Action::ExecuteCode(String::from(
                    "(println \"lambert, lambert\")",
                )),
            ),
        ];

        let result = get_defined_actions(&mut interpreter).unwrap();

        nia_assert_equal(expected.len(), result.len());

        for (expected, result) in expected.into_iter().zip(result.into_iter()) {
            println!("{:?}", expected);
            println!("{:?}", result);
            nia_assert_equal(expected, result);
        }
    }
}