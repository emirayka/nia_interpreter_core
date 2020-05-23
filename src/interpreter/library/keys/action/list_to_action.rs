use crate::Action;
use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::DEFINED_ACTIONS_ROOT_VARIABLE_NAME;

use crate::library;

fn try_remove_first_item(
    list: &mut Vec<Value>,
    msg: &str,
) -> Result<Value, Error> {
    if list.len() == 0 {
        return Error::invalid_argument_error(msg).into();
    }

    return Ok(list.remove(0));
}

fn check_list_has_no_items_left(
    list: &Vec<Value>,
    msg: &str,
) -> Result<(), Error> {
    if list.len() != 0 {
        return Error::invalid_argument_error(msg).into();
    }

    Ok(())
}

fn parse_key_press_action(
    mut action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let key_code_value = try_remove_first_item(
        &mut action_vector,
        "Key press action list must have two items exactly to be considered as action.",
    )?;

    check_list_has_no_items_left(
        &action_vector,
        "Key press action list must have two items exactly to be considered as action.",
    )?;

    let key_code = library::read_as_i64(key_code_value)? as i32;

    Ok(Action::KeyPress(key_code))
}

fn parse_key_click_action(
    mut action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let key_code_value = try_remove_first_item(
        &mut action_vector,
        "Key click action list must have at least one item, to be considered as action.",
    )?;

    check_list_has_no_items_left(
        &action_vector,
        "Key click action list must have at least one item, to be considered as action.",
    )?;

    let key_code = library::read_as_i64(key_code_value)? as i32;

    Ok(Action::KeyClick(key_code))
}

fn parse_key_release_action(
    mut action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let key_code_value = try_remove_first_item(
        &mut action_vector,
        "List must take at least one item, to be considered as action.",
    )?;

    check_list_has_no_items_left(
        &action_vector,
        "List must take at least one item, to be considered as action.",
    )?;

    let key_code = library::read_as_i64(key_code_value)? as i32;

    Ok(Action::KeyRelease(key_code))
}

fn parse_mouse_button_press_action(
    mut action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let button_code_value = try_remove_first_item(
        &mut action_vector,
        "List must have two items exactly, to be considered as action.",
    )?;

    check_list_has_no_items_left(
        &action_vector,
        "List must have two items exactly, to be considered as action.",
    )?;

    let key_code = library::read_as_i64(button_code_value)? as i32;

    Ok(Action::MouseButtonPress(key_code))
}

fn parse_mouse_button_click_action(
    mut action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let button_code_value = try_remove_first_item(
        &mut action_vector,
        "List must have two items, to be considered as action.",
    )?;

    check_list_has_no_items_left(
        &action_vector,
        "List must have two items, to be considered as action.",
    )?;

    let button_code = library::read_as_i64(button_code_value)? as i32;

    Ok(Action::MouseButtonClick(button_code))
}

fn parse_mouse_button_release_action(
    mut action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let button_code_value = try_remove_first_item(
        &mut action_vector,
        "List must have two items exactly, to be considered as action.",
    )?;

    check_list_has_no_items_left(
        &action_vector,
        "List must have two items exactly, to be considered as action.",
    )?;

    let key_code = library::read_as_i64(button_code_value)? as i32;

    Ok(Action::MouseButtonRelease(key_code))
}

fn parse_mouse_absolute_move_action(
    mut action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let x = try_remove_first_item(
        &mut action_vector,
        "List must have three items exactly, to be considered as mouse absolute move action.",
    )?;

    let y = try_remove_first_item(
        &mut action_vector,
        "List must have three items exactly, to be considered as mouse absolute move action.",
    )?;

    check_list_has_no_items_left(
        &action_vector,
        "List must have three items exactly, to be considered as mouse absolute move action.",
    )?;

    let x = library::read_as_i64(x)? as i32;
    let y = library::read_as_i64(y)? as i32;

    Ok(Action::MouseAbsoluteMove(x, y))
}

fn parse_mouse_relative_move_action(
    mut action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let dx = try_remove_first_item(
        &mut action_vector,
        "List must have three items exactly, to be considered as mouse relative move action.",
    )?;

    let dy = try_remove_first_item(
        &mut action_vector,
        "List must have three items exactly, to be considered as mouse relative move action.",
    )?;

    check_list_has_no_items_left(
        &action_vector,
        "List must have three items exactly, to be considered as mouse relative move action.",
    )?;

    let dx = library::read_as_i64(dx)? as i32;
    let dy = library::read_as_i64(dy)? as i32;

    Ok(Action::MouseRelativeMove(dx, dy))
}

fn parse_text_type_action(
    interpreter: &mut Interpreter,
    mut action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let text = try_remove_first_item(
        &mut action_vector,
        "List must have two items exactly, to be considered as text type action.",
    )?;

    check_list_has_no_items_left(
        &action_vector,
        "List must have two items exactly, to be considered as text type action.",
    )?;

    let text = library::read_as_string(interpreter, text)?.clone();

    Ok(Action::TextType(text))
}

fn parse_execute_code_action(
    interpreter: &mut Interpreter,
    mut action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let code = try_remove_first_item(
        &mut action_vector,
        "List must have two items exactly, to be considered as execute code action.",
    )?;

    check_list_has_no_items_left(
        &action_vector,
        "List must have two items exactly, to be considered as execute code action.",
    )?;

    let code = library::read_as_string(interpreter, code)?.clone();

    Ok(Action::ExecuteCode(code))
}

fn parse_execute_function_action(
    interpreter: &mut Interpreter,
    mut action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let function_name = try_remove_first_item(
        &mut action_vector,
        "List must have two items exactly, to be considered as execute function action.",
    )?;

    check_list_has_no_items_left(
        &action_vector,
        "List must have two items exactly, to be considered as execute function action.",
    )?;

    let function_name =
        library::read_as_string(interpreter, function_name)?.clone();

    Ok(Action::ExecuteFunction(function_name))
}

fn parse_execute_os_command_action(
    interpreter: &mut Interpreter,
    mut action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let os_command = try_remove_first_item(
        &mut action_vector,
        "List must have two items exactly, to be considered as execute os command action.",
    )?;

    check_list_has_no_items_left(
        &action_vector,
        "List must have two items exactly, to be considered as execute os command action.",
    )?;

    let os_command = library::read_as_string(interpreter, os_command)?.clone();

    Ok(Action::ExecuteOSCommand(os_command))
}

fn parse_wait_action(
    interpreter: &mut Interpreter,
    mut action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let ms_amount = try_remove_first_item(
        &mut action_vector,
        "List must have two items exactly, to be considered as wait action.",
    )?;

    check_list_has_no_items_left(
        &action_vector,
        "List must have two items exactly, to be considered as wait action.",
    )?;

    let ms_amount = library::read_as_i64(ms_amount)?.clone() as i32;

    Ok(Action::Wait(ms_amount))
}

fn parse_execute_function_value_action(
    interpreter: &mut Interpreter,
    mut action_vector: Vec<Value>,
) -> Result<Action, Error> {
    let function_value = try_remove_first_item(
        &mut action_vector,
        "List must have two items exactly, to be considered as execute-function-value action.",
    )?;

    library::check_value_is_function(function_value);

    Ok(Action::ExecuteFunctionValue(function_value))
}

pub fn list_to_action(
    interpreter: &mut Interpreter,
    action_list: Value,
) -> Result<Action, Error> {
    let mut action_vector = library::read_as_vector(interpreter, action_list)?;

    let action_type_value = try_remove_first_item(
        &mut action_vector,
        "List must take at least one item, to be considered as an action.",
    )?;

    let action_type_symbol_id = library::read_as_symbol_id(action_type_value)?;
    let action_type_symbol_name =
        interpreter.get_symbol_name(action_type_symbol_id)?.as_str();

    let action = match action_type_symbol_name {
        "key-press" => parse_key_press_action(action_vector)?,
        "key-click" => parse_key_click_action(action_vector)?,
        "key-release" => parse_key_release_action(action_vector)?,

        "mouse-button-press" => parse_mouse_button_press_action(action_vector)?,
        "mouse-button-click" => parse_mouse_button_click_action(action_vector)?,
        "mouse-button-release" => {
            parse_mouse_button_release_action(action_vector)?
        }

        "mouse-absolute-move" => {
            parse_mouse_absolute_move_action(action_vector)?
        }
        "mouse-relative-move" => {
            parse_mouse_relative_move_action(action_vector)?
        }

        "text-type" => parse_text_type_action(interpreter, action_vector)?,
        "execute-code" => {
            parse_execute_code_action(interpreter, action_vector)?
        }
        "execute-function" => {
            parse_execute_function_action(interpreter, action_vector)?
        }
        "execute-os-command" => {
            parse_execute_os_command_action(interpreter, action_vector)?
        }
        "execute-function-value" => {
            parse_execute_function_value_action(interpreter, action_vector)?
        }
        "wait" => parse_wait_action(interpreter, action_vector)?,

        _ => {
            return Error::invalid_argument_error(format!(
                "Unknown action type: {}.",
                action_type_symbol_name
            ))
            .into();
        }
    };

    Ok(action)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::utils;

    #[test]
    fn converts_to_action_correctly() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (Action::KeyPress(1), r#"'(key-press 1)"#),
            (Action::KeyClick(2), r#"'(key-click 2)"#),
            (Action::KeyRelease(3), r#"'(key-release 3)"#),
            (Action::MouseButtonPress(4), r#"'(mouse-button-press 4)"#),
            (Action::MouseButtonClick(5), r#"'(mouse-button-click 5)"#),
            (
                Action::MouseButtonRelease(6),
                r#"'(mouse-button-release 6)"#,
            ),
            (
                Action::MouseAbsoluteMove(100, 100),
                r#"'(mouse-absolute-move 100 100)"#,
            ),
            (
                Action::MouseRelativeMove(100, 100),
                r#"'(mouse-relative-move 100 100)"#,
            ),
            (
                Action::TextType(String::from("nya")),
                r#"'(text-type "nya")"#,
            ),
            (
                Action::ExecuteCode(String::from("(println \"kek\")")),
                r#"'(execute-code "(println \"kek\")")"#,
            ),
            (
                Action::ExecuteFunction(String::from("test")),
                r#"'(execute-function "test")"#,
            ),
            (
                Action::ExecuteOSCommand(String::from("echo nya")),
                r#"'(execute-os-command "echo nya")"#,
            ),
            (Action::Wait(1000), r#"'(wait 1000)"#),
        ];

        for (expected, code) in specs {
            let value = interpreter.execute_in_main_environment(code).unwrap();
            let result = list_to_action(&mut interpreter, value).unwrap();

            nia_assert_equal(expected, result);
        }
    }

    #[test]
    fn converts_to_execute_function_value_correctly() {
        let mut interpreter = Interpreter::new();

        let execute_function_value_symbol_value =
            interpreter.intern_symbol_value("execute-function-value");
        let function = interpreter.execute_in_main_environment("#()").unwrap();
        let list_representation = interpreter
            .vec_to_list(vec![execute_function_value_symbol_value, function]);

        let expected = Action::ExecuteFunctionValue(function);
        let result =
            list_to_action(&mut interpreter, list_representation).unwrap();

        nia_assert_equal(expected, result);
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_argument_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            r#"'(unknown-action 1)"#,
            r#"'(key-press)"#,
            r#"'(key-press 1 2)"#,
            r#"'(key-click)"#,
            r#"'(key-click 1 2)"#,
            r#"'(key-release)"#,
            r#"'(key-release 1 3)"#,
            r#"'(mouse-button-press)"#,
            r#"'(mouse-button-press 3 4)"#,
            r#"'(mouse-button-click)"#,
            r#"'(mouse-button-click 5 6)"#,
            r#"'(mouse-button-release)"#,
            r#"'(mouse-button-release 6 7)"#,
            r#"'(mouse-absolute-move)"#,
            r#"'(mouse-absolute-move 100)"#,
            r#"'(mouse-absolute-move 100 100 100 )"#,
            r#"'(mouse-relative-move)"#,
            r#"'(mouse-relative-move 100)"#,
            r#"'(mouse-relative-move 100 100 100)"#,
            r#"'(text-type)"#,
            r#"'(text-type "nya" "nya")"#,
            r#"'(execute-code)"#,
            r#"'(execute-code "(println \"kek\")" "(println \"kek\")")"#,
            r#"'(execute-os-command)"#,
            r#"'(execute-os-command "echo-nya" "echo-nya")"#,
            r#"'(wait)"#,
            r#"'(wait 1000 1000)"#,
            // todo: add argument tests ???
        ];

        for spec in specs {
            let value = interpreter.execute_in_main_environment(spec).unwrap();
            let result = list_to_action(&mut interpreter, value);

            utils::assert_invalid_argument_error(&result)
        }
    }
}
