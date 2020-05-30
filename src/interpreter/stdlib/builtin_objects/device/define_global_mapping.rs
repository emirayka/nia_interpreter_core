use crate::Action;
use crate::EnvironmentId;
use crate::Error;
use crate::Interpreter;
use crate::Key;
use crate::KeyChord;
use crate::Mapping;
use crate::Value;

use crate::library;

fn string_to_key(s: &str) -> Result<Key, Error> {
    let parts = s.split(":").collect::<Vec<&str>>();

    match parts.len() {
        1 => {
            let key_code_name = parts[0];
            let key_code =
                nia_event_codes::map_string_to_key_code(key_code_name)
                    .ok_or_else(|| Error::invalid_argument_error(""))?;

            Ok(nia_key!(key_code))
        }
        2 => {
            let device_id = parts[0]
                .parse::<u32>()
                .map_err(|_| Error::invalid_argument_error(""))?
                as i32;

            let key_code_name = parts[1];
            let key_code =
                nia_event_codes::map_string_to_key_code(key_code_name)
                    .ok_or_else(|| Error::invalid_argument_error(""))?;

            Ok(nia_key!(device_id, key_code))
        }
        _ => Error::invalid_argument_error("").into(),
    }
}

fn string_to_key_chord(s: &str) -> Result<KeyChord, Error> {
    let mut keys = s
        .split("+")
        .map(string_to_key)
        .collect::<Result<Vec<Key>, Error>>()?;

    if keys.len() == 0 {
        return Error::invalid_argument_error("").into();
    }

    let key = keys.remove(keys.len() - 1);
    let modifiers = keys;

    Ok(KeyChord::new(modifiers, key))
}

pub fn define_global_mapping(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `device:define-global-mapping' takes two arguments exactly.",
        )
            .into();
    }

    let values = values;

    let key_chord_part = library::read_as_string(interpreter, values[0])?;
    let action_part = values[1];

    library::check_value_is_function(action_part)?;

    let key_chords = key_chord_part
        .split(" ")
        .filter(|s| s.len() > 0)
        .map(string_to_key_chord)
        .collect::<Result<Vec<KeyChord>, Error>>()?;

    if key_chords.len() == 0 {
        return Error::invalid_argument_error("").into();
    }

    let action = Action::ExecuteFunctionValue(action_part);

    let mapping = Mapping::new(key_chords, action);

    // todo: optimize
    library::define_global_mapping(interpreter, &mapping)?;

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;
    use crate::GLOBAL_MAP_ROOT_VARIABLE_NAME;

    #[test]
    fn defines_new_mappings() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (GLOBAL_MAP_ROOT_VARIABLE_NAME, "'()"),
            (
                "(device:define-global-mapping \"CtrlL+b\" #(+ 1 2))",
                "nil",
            ),
            (GLOBAL_MAP_ROOT_VARIABLE_NAME, "(list:new (list:new (list:new (list:new 29 48)) 'execute-function-value #(+ 1 2)))"),
            (
                "(device:define-global-mapping \"CtrlL+c   CtrlL+b\" #())",
                "nil",
            ),
            (
                GLOBAL_MAP_ROOT_VARIABLE_NAME,
                "(list:new (list:new (list:new (list:new 29 46) (list:new 29 48)) 'execute-function-value #()) (list:new (list:new (list:new 29 48)) 'execute-function-value #(+ 1 2)))",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn defines_new_mappings_with_device_identifiers() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (GLOBAL_MAP_ROOT_VARIABLE_NAME, "'()"),
            (
                "(device:define-global-mapping \"0:CtrlL+0:b\" #(+ 1 2))",
                "nil",
            ),
            (
                GLOBAL_MAP_ROOT_VARIABLE_NAME,
                "(list:new (list:new (list:new (list:new (list:new 0 29) (list:new 0 48))) 'execute-function-value #(+ 1 2)))",
            ),
            (
                "(device:define-global-mapping \"1:CtrlL+1:c 1:CtrlL+1:b\" #())",
                "nil",
            ),
            (
                GLOBAL_MAP_ROOT_VARIABLE_NAME,
                "(list:new (list:new (list:new (list:new (list:new 1 29) (list:new 1 46)) (list:new (list:new 1 29) (list:new 1 48))) 'execute-function-value #()) (list:new (list:new (list:new (list:new 0 29) (list:new 0 48))) 'execute-function-value #(+ 1 2)))",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(device:define-global-mapping 1 #())",
            "(device:define-global-mapping 1.1 #())",
            "(device:define-global-mapping #f #())",
            "(device:define-global-mapping #t #())",
            "(device:define-global-mapping :keyword #())",
            "(device:define-global-mapping 'symbol #())",
            "(device:define-global-mapping '(1 2) #())",
            "(device:define-global-mapping {} #())",
            "(device:define-global-mapping #() #())",
            "(device:define-global-mapping \"q\" 1)",
            "(device:define-global-mapping \"q\" 1.1)",
            "(device:define-global-mapping \"q\" #t)",
            "(device:define-global-mapping \"q\" #f)",
            "(device:define-global-mapping \"q\" \"string\")",
            "(device:define-global-mapping \"q\" :keyword)",
            "(device:define-global-mapping \"q\" 'symbol)",
            "(device:define-global-mapping \"q\" '())",
            "(device:define-global-mapping \"q\" {})",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(device:define-global-mapping)",
            "(device:define-global-mapping \"path\")",
            "(device:define-global-mapping \"path\" \"name\" '())",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_key_chord_sequence_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(device:define-global-mapping "" #())"#,
            r#"(device:define-global-mapping "unexistingkey" #())"#,
            r#"(device:define-global-mapping "+" #())"#,
            r#"(device:define-global-mapping "++" #())"#,
            r#"(device:define-global-mapping "a+" #())"#,
            r#"(device:define-global-mapping "a++" #())"#,
            r#"(device:define-global-mapping "1::2" #())"#,
            r#"(device:define-global-mapping "1:2:3" #())"#,
            r#"(device:define-global-mapping ":" #())"#,
            r#"(device:define-global-mapping "::" #())"#,
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
