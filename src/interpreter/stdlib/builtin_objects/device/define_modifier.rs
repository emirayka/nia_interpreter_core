use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::library;

pub fn define_modifier(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `device:define-modifier' takes two arguments exactly.",
        )
        .into();
    }

    let key_part = library::read_as_string(interpreter, values[0])?;
    let alias_part = values[1];

    library::check_value_is_string(alias_part)?;

    let key = key_part.split(":").collect::<Vec<&str>>();

    if key.len() < 1 || key.len() > 2 {
        return Error::invalid_argument_error("").into();
    }

    let device_id = if key.len() == 2 {
        let device_id = key[0]
            .parse::<i64>()
            .map_err(|_| Error::invalid_argument_error(""))?;

        Some(Value::Integer(device_id))
    } else {
        None
    };

    let key_code = if key.len() == 2 {
        nia_event_codes::map_string_to_key_code(key[1])
            .ok_or_else(|| Error::invalid_argument_error(""))?
    } else {
        nia_event_codes::map_string_to_key_code(key[0])
            .ok_or_else(|| Error::invalid_argument_error(""))?
    } as i64;

    let key_code = Value::Integer(key_code);

    library::define_modifier_with_values(
        interpreter,
        device_id,
        key_code,
        alias_part,
    )?;

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;
    use crate::DEFINED_MODIFIERS_ROOT_VARIABLE_NAME;

    #[test]
    fn defines_new_modifiers() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (DEFINED_MODIFIERS_ROOT_VARIABLE_NAME, "'()"),
            ("(device:define-modifier \"CtrlL\" \"Control\")", "nil"),
            (DEFINED_MODIFIERS_ROOT_VARIABLE_NAME, "'((29 \"Control\"))"),
            ("(device:define-modifier \"0:MetaL\" \"Meta\")", "nil"),
            (
                DEFINED_MODIFIERS_ROOT_VARIABLE_NAME,
                r#"'((0 125 "Meta") (29 "Control"))"#,
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(device:define-modifier 1 \"alias\")",
            "(device:define-modifier 1.1 \"alias\")",
            "(device:define-modifier #t \"alias\")",
            "(device:define-modifier #f \"alias\")",
            "(device:define-modifier :keyword \"alias\")",
            "(device:define-modifier 'symbol \"alias\")",
            "(device:define-modifier '(list:new) \"alias\")",
            "(device:define-modifier {} \"alias\")",
            "(device:define-modifier #() \"alias\")",
            "(device:define-modifier \"a\" 1)",
            "(device:define-modifier \"a\" 1.1)",
            "(device:define-modifier \"a\" #t)",
            "(device:define-modifier \"a\" #f)",
            "(device:define-modifier \"a\" :keyword)",
            "(device:define-modifier \"a\" 'symbol)",
            "(device:define-modifier \"a\" '(list:new))",
            "(device:define-modifier \"a\" {})",
            "(device:define-modifier \"a\" #())",
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
            "(device:define-modifier)",
            "(device:define-modifier \"a\")",
            "(device:define-modifier \"b\" \"alias\" 2)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
