use std::collections::HashMap;

use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn define_modifier(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `keyboard:define-modifier' takes one argument exactly."
        ).into_result();
    }

    let mut values = values;

    let key_chord_part = nia_events::str_to_key_chord_part(
        library::read_as_string(
            interpreter,
            values.remove(0)
        )?,
        &HashMap::new() // todo: fix
    ).map_err(|_| Error::invalid_argument_error(
        "Cannot parse key chord part."
    ))?;

    let key_chord_value = library::key_chord_part_to_list(
        interpreter,
        key_chord_part
    );

    library::add_value_to_root_list(
        interpreter,
        "modifiers",
        key_chord_value
    )?;

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn defines_new_modifiers() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("modifiers", "'()"),
            ("(keyboard:define-modifier \"LeftControl\")", "nil"),
            ("modifiers", "'(29)"),
            ("(keyboard:define-modifier \"0:LeftMeta\")", "nil"),
            ("modifiers", "'((0 125) 29)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(keyboard:define-modifier 1)",
            "(keyboard:define-modifier 1.1)",
            "(keyboard:define-modifier #t)",
            "(keyboard:define-modifier #f)",
            "(keyboard:define-modifier :keyword)",
            "(keyboard:define-modifier 'symbol)",
            "(keyboard:define-modifier '(list))",
            "(keyboard:define-modifier {})",
            "(keyboard:define-modifier #())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(keyboard:define-modifier)",
            "(keyboard:define-modifier \"path\" \"name\")",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
