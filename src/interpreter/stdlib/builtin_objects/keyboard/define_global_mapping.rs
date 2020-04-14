use std::collections::HashMap;

use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

use nia_events::{KeyChord, KeyChordPart};

fn key_chord_part_to_list(
    interpreter: &mut Interpreter,
    key_chord_part: &KeyChordPart
) -> Value {
    match key_chord_part {
        KeyChordPart::Key1(key_id) => {
            Value::Integer(key_id.get_id() as i64)
        },
        KeyChordPart::Key2(keyboard_id, key_id) => {
            interpreter.vec_to_list(
                vec!(
                    Value::Integer(keyboard_id.get_id() as i64),
                    Value::Integer(key_id.get_id() as i64)
                )
            )
        }
    }
}

fn key_chord_to_list(
    interpreter: &mut Interpreter,
    key_chord: KeyChord
) -> Value {
    let mut vector = Vec::new();

    for modifier in key_chord.get_modifiers() {
        vector.push(key_chord_part_to_list(interpreter, modifier));
    }

    vector.push(key_chord_part_to_list(interpreter, key_chord.get_key()));

    interpreter.vec_to_list(vector)
}

fn key_chords_to_list(
    interpreter: &mut Interpreter,
    key_chords: Vec<KeyChord>
) -> Value {
    let mut vector = Vec::new();

    for key_chord in key_chords {
        vector.push(key_chord_to_list(interpreter, key_chord));
    }

    interpreter.vec_to_list(vector)
}

pub fn define_global_mapping(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `keyboard:define-global-mapping' takes two arguments exactly."
        ).into_result();
    }

    let mut values = values;

    let mapping = library::read_as_string(interpreter, values.remove(0))?.clone(); // todo: fix
    let function_id = library::read_as_function_id(interpreter, values.remove(0))?;

    let global_map_cons_cell = library::get_root_variable(
        interpreter,
        "global-map"
    )?;

    let mut key_chords = Vec::new();

    for mapping_part in mapping.split(" ") {
        let key_chord = nia_events::str_to_key_chord(
            mapping_part,
            &HashMap::new() // todo: change
        ).map_err(|_| interpreter.make_invalid_argument_error(
            "Invalid key chord."
        ))?;

        key_chords.push(key_chord);
    }

    let mut values = key_chords_to_list(interpreter, key_chords);

    let new_mapping = interpreter.vec_to_list(vec!(values, function_id.to_value()));
    let new_list = interpreter.make_cons_value(new_mapping, global_map_cons_cell);

    library::set_root_variable(
        interpreter,
        "global-map",
        new_list
    );

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn defines_new_mappings() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("global-map", "'()"),
            ("(keyboard:define-global-mapping \"LeftControl+b\" #(+ 1 2))", "nil"),
            ("global-map", "(list (list (list (list 29 48)) #(+ 1 2)))"),
            ("(keyboard:define-global-mapping \"LeftControl+c LeftControl+b\" #())", "nil"),
            ("global-map", "(list (list (list (list 29 46) (list 29 48)) #()) (list (list (list 29 48)) #(+ 1 2)))"),
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
            "(keyboard:define-global-mapping 1 #())",
            "(keyboard:define-global-mapping 1.1 #())",
            "(keyboard:define-global-mapping #f #())",
            "(keyboard:define-global-mapping #t #())",
            "(keyboard:define-global-mapping :keyword #())",
            "(keyboard:define-global-mapping 'symbol #())",
            "(keyboard:define-global-mapping '(1 2) #())",
            "(keyboard:define-global-mapping {} #())",
            "(keyboard:define-global-mapping #() #())",

            "(keyboard:define-global-mapping \"q\" 1)",
            "(keyboard:define-global-mapping \"q\" 1.1)",
            "(keyboard:define-global-mapping \"q\" #t)",
            "(keyboard:define-global-mapping \"q\" #f)",
            "(keyboard:define-global-mapping \"q\" \"string\")",
            "(keyboard:define-global-mapping \"q\" :keyword)",
            "(keyboard:define-global-mapping \"q\" 'symbol)",
            "(keyboard:define-global-mapping \"q\" '())",
            "(keyboard:define-global-mapping \"q\" {})",
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
            "(keyboard:define-global-mapping)",
            "(keyboard:define-global-mapping \"path\")",
            "(keyboard:define-global-mapping \"path\" \"name\" '())",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
