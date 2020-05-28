use crate::Action;
use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::DEFINED_ACTIONS_ROOT_VARIABLE_NAME;

use crate::library;

fn try_remove_first_item<S>(
    list: &mut Vec<Value>,
    msg: S,
) -> Result<Value, Error>
where
    S: Into<String>,
{
    if list.len() == 0 {
        return Error::invalid_argument_error(msg).into();
    }

    return Ok(list.remove(0));
}

fn check_list_has_no_items_left<S>(
    list: &Vec<Value>,
    msg: S,
) -> Result<(), Error>
where
    S: Into<String>,
{
    if list.len() != 0 {
        return Error::invalid_argument_error(msg).into();
    }

    Ok(())
}

macro_rules! make_one_integer_item_action_parser {
    ($parser_name:ident, $action_name:expr, $action_variant:path) => {
        fn $parser_name(
            mut action_vector: Vec<Value>,
        ) -> Result<Action, Error> {
            let value = try_remove_first_item(
                &mut action_vector,
                format!("{} list must have two items exactly to be considered as action.", $action_name),
            )?;

            check_list_has_no_items_left(
                &action_vector,
                format!("{} list must have two items exactly to be considered as action.", $action_name),
            )?;

            let value = library::read_as_i64(value)? as i32;

            Ok($action_variant(value))
        }
    }
}

macro_rules! make_one_string_item_action_parser {
    ($parser_name:ident, $action_name:expr, $action_variant:path) => {
        fn $parser_name(
            interpreter: &mut Interpreter,
            mut action_vector: Vec<Value>,
        ) -> Result<Action, Error> {
            let value = try_remove_first_item(
                &mut action_vector,
                format!("{} list must have two items exactly to be considered as action.", $action_name),
            )?;

            check_list_has_no_items_left(
                &action_vector,
                format!("{} list must have two items exactly to be considered as action.", $action_name),
            )?;

            let value = library::read_as_string(interpreter, value)?.clone();

            Ok($action_variant(value))
        }
    }
}

macro_rules! make_two_integers_item_action_parser {
    ($parser_name:ident, $action_name:expr, $action_variant:path) => {
        fn $parser_name(
            mut action_vector: Vec<Value>,
        ) -> Result<Action, Error> {
            let value1 = try_remove_first_item(
                &mut action_vector,
                format!("{} list must have three items exactly to be considered as action.", $action_name),
            )?;

            let value2 = try_remove_first_item(
                &mut action_vector,
                format!("{} list must have three items exactly to be considered as action.", $action_name),
            )?;

            check_list_has_no_items_left(
                &action_vector,
                format!("{} list must have three items exactly to be considered as action.", $action_name),
            )?;

            let value1 = library::read_as_i64(value1)? as i32;
            let value2 = library::read_as_i64(value2)? as i32;

            Ok($action_variant(value1, value2))
        }
    }
}

#[rustfmt::skip]
make_one_integer_item_action_parser!(parse_key_press_action, "Key press action", Action::KeyPress);
#[rustfmt::skip]
make_one_integer_item_action_parser!(parse_key_click_action, "Key click action", Action::KeyClick );
#[rustfmt::skip]
make_one_integer_item_action_parser!(parse_key_release_action, "Key release action", Action::KeyRelease );

#[rustfmt::skip]
make_one_integer_item_action_parser!(parse_mouse_button_press_action, "Key press action", Action::MouseButtonPress);
#[rustfmt::skip]
make_one_integer_item_action_parser!(parse_mouse_button_click_action, "Mouse button click action", Action::MouseButtonClick);
#[rustfmt::skip]
make_one_integer_item_action_parser!(parse_mouse_button_release_action, "Mouse button release action", Action::MouseButtonRelease);

#[rustfmt::skip]
make_one_integer_item_action_parser!(parse_text_key_click_action, "Text key click action", Action::TextKeyClick);
#[rustfmt::skip]
make_one_integer_item_action_parser!(parse_number_key_click_action, "Number key click action", Action::NumberKeyClick);
#[rustfmt::skip]
make_one_integer_item_action_parser!(parse_function_key_click_action, "Function key click action", Action::FunctionKeyClick);
#[rustfmt::skip]
make_one_integer_item_action_parser!(parse_control_key_click_action, "Control key click action", Action::ControlKeyClick);
#[rustfmt::skip]
make_one_integer_item_action_parser!(parse_kp_key_click_action, "KP key click action", Action::KPKeyClick);
#[rustfmt::skip]
make_one_integer_item_action_parser!(parse_multimedia_key_click_action, "Multimedia key click action", Action::MultimediaKeyClick);
#[rustfmt::skip]
make_one_integer_item_action_parser!(parse_mouse_button_key_click_action, "Mouse button key click action", Action::MouseButtonKeyClick);

#[rustfmt::skip]
make_two_integers_item_action_parser!(parse_mouse_absolute_move_action, "Mouse absolute move action", Action::MouseAbsoluteMove);
#[rustfmt::skip]
make_two_integers_item_action_parser!(parse_mouse_relative_move_action, "Mouse relative move action", Action::MouseRelativeMove);

#[rustfmt::skip]
make_one_integer_item_action_parser!(parse_wait_action, "Wait action", Action::Wait);
#[rustfmt::skip]
make_one_string_item_action_parser!(parse_text_type_action, "Text type action", Action::TextType);
#[rustfmt::skip]
make_one_string_item_action_parser!(parse_execute_function_action, "Execute function action", Action::ExecuteFunction);
#[rustfmt::skip]
make_one_string_item_action_parser!(parse_execute_os_command_action, "Execute OS command action", Action::ExecuteOSCommand);
#[rustfmt::skip]
make_one_string_item_action_parser!(parse_execute_code_action, "Execute code action", Action::ExecuteCode);
#[rustfmt::skip]
make_one_string_item_action_parser!(parse_execute_named_action_action, "Execute named action", Action::ExecuteNamedAction);

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

    #[rustfmt::skip]
    let action = match action_type_symbol_name {
        "key-press" => parse_key_press_action(action_vector)?,
        "key-click" => parse_key_click_action(action_vector)?,
        "key-release" => parse_key_release_action(action_vector)?,

        "mouse-button-press" => parse_mouse_button_press_action(action_vector)?,
        "mouse-button-click" => parse_mouse_button_click_action(action_vector)?,
        "mouse-button-release" => parse_mouse_button_release_action(action_vector)?,
        
        "text-key-click" => parse_text_key_click_action(action_vector)?,
        "number-key-click" => parse_number_key_click_action(action_vector)?,
        "function-key-click" => parse_function_key_click_action(action_vector)?,
        "control-key-click" => parse_control_key_click_action(action_vector)?,
        "kp-key-click" => parse_kp_key_click_action(action_vector)?,
        "multimedia-key-click" => parse_multimedia_key_click_action(action_vector)?,
        "mouse-button-key-click" => parse_mouse_button_key_click_action(action_vector)?,

        "mouse-absolute-move" => parse_mouse_absolute_move_action(action_vector)?,
        "mouse-relative-move" => parse_mouse_relative_move_action(action_vector)?,

        "wait" => parse_wait_action(action_vector)?,
        "text-type" => parse_text_type_action(interpreter, action_vector)?,
        
        "execute-code" => parse_execute_code_action(interpreter, action_vector)?,
        "execute-function" => parse_execute_function_action(interpreter, action_vector)?,
        "execute-os-command" => parse_execute_os_command_action(interpreter, action_vector)?,
        "execute-named-action" => parse_execute_named_action_action(interpreter, action_vector)?,
        
        "execute-function-value" => parse_execute_function_value_action(interpreter, action_vector)?,

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

        #[rustfmt::skip]
        let specs = vec![
            (Action::KeyPress(1), r#"'(key-press 1)"#),
            (Action::KeyClick(2), r#"'(key-click 2)"#),
            (Action::KeyRelease(3), r#"'(key-release 3)"#),
            
            (Action::MouseButtonPress(4), r#"'(mouse-button-press 4)"#),
            (Action::MouseButtonClick(5), r#"'(mouse-button-click 5)"#),
            (Action::MouseButtonRelease(6), r#"'(mouse-button-release 6)"#),
            
            (Action::TextKeyClick(7), r#"'(text-key-click 7)"#),
            (Action::NumberKeyClick(7), r#"'(number-key-click 7)"#),
            (Action::FunctionKeyClick(7), r#"'(function-key-click 7)"#),
            (Action::ControlKeyClick(7), r#"'(control-key-click 7)"#),
            (Action::KPKeyClick(7), r#"'(kp-key-click 7)"#),
            (Action::MultimediaKeyClick(7), r#"'(multimedia-key-click 7)"#),
            (Action::MouseButtonKeyClick(7), r#"'(mouse-button-key-click 7)"#),
            
            (Action::MouseAbsoluteMove(100, 100), r#"'(mouse-absolute-move 100 100)"#),
            (Action::MouseRelativeMove(100, 100), r#"'(mouse-relative-move 100 100)"#),

            (Action::Wait(1000), r#"'(wait 1000)"#),
            (Action::TextType(String::from("nya")), r#"'(text-type "nya")"#),
            (Action::ExecuteCode(String::from("(println \"kek\")")), r#"'(execute-code "(println \"kek\")")"#),
            (Action::ExecuteFunction(String::from("test")), r#"'(execute-function "test")"#),
            (Action::ExecuteOSCommand(String::from("echo nya")), r#"'(execute-os-command "echo nya")"#),
            (Action::ExecuteNamedAction(String::from("print-nya")), r#"'(execute-named-action "print-nya")"#),
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

        #[rustfmt::skip]
        let specs = vec![
            r#"'(unknown-action 1)"#,
            
            r#"'(key-press)"#,
            r#"'(key-press 1 2)"#,
            r#"'(key-click)"#,
            r#"'(key-click 1 2)"#,
            r#"'(key-release)"#,
            r#"'(key-release 1 2)"#,
            
            r#"'(mouse-button-press)"#,
            r#"'(mouse-button-press 1 2)"#,
            r#"'(mouse-button-click)"#,
            r#"'(mouse-button-click 1 2)"#,
            r#"'(mouse-button-release)"#,
            r#"'(mouse-button-release 1 2)"#,
            
            r#"'(text-key-click)"#,
            r#"'(text-key-click 1 2)"#,
            r#"'(number-key-click)"#,
            r#"'(number-key-click 1 2)"#,
            r#"'(function-key-click)"#,
            r#"'(function-key-click 1 2)"#,
            r#"'(control-key-click)"#,
            r#"'(control-key-click 1 2)"#,
            r#"'(kp-key-click)"#,
            r#"'(kp-key-click 1 2)"#,
            r#"'(multimedia-key-click)"#,
            r#"'(multimedia-key-click 1 2)"#,
            r#"'(mouse-button-key-click)"#,
            r#"'(mouse-button-key-click 1 2)"#,
            
            r#"'(mouse-absolute-move)"#,
            r#"'(mouse-absolute-move 100)"#,
            r#"'(mouse-absolute-move 100 100 100)"#,
            r#"'(mouse-relative-move)"#,
            r#"'(mouse-relative-move 100)"#,
            
            r#"'(mouse-relative-move 100 100 100)"#,
            r#"'(text-type)"#,
            r#"'(text-type "nya" "nya")"#,
            r#"'(execute-code)"#,
            r#"'(execute-code "(println \"kek\")" "(println \"kek\")")"#,
            r#"'(execute-os-command)"#,
            r#"'(execute-os-command "echo-nya" "echo-nya")"#,
            r#"'(execute-named-action)"#,
            r#"'(execute-named-action "print-nya" "print-nya")"#,
            r#"'(wait)"#,
            r#"'(wait 1000 1000)"#,
        ];

        for spec in specs {
            let value = interpreter.execute_in_main_environment(spec).unwrap();
            let result = list_to_action(&mut interpreter, value);

            utils::assert_invalid_argument_error(&result)
        }
    }
}
