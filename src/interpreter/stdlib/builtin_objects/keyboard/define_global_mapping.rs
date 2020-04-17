use std::collections::HashMap;

use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

use nia_events::{KeyChord, KeyboardId};

fn key_chords_to_list(
    interpreter: &mut Interpreter,
    key_chords: Vec<KeyChord>
) -> Value {
    let mut vector = Vec::new();

    for key_chord in key_chords {
        vector.push(library::key_chord_to_list(interpreter, key_chord));
    }

    interpreter.vec_to_list(vector)
}

fn read_registered_keyboards(
    interpreter: &mut Interpreter
) -> Result<HashMap<String, KeyboardId>, Error> {
    let mut result = HashMap::new();

    let registered_keyboards = library::get_root_variable(
        interpreter,
        "registered-keyboards"
    )?;

    let registered_keyboards = library::read_as_vector(
        interpreter,
        registered_keyboards
    )?;

    for (index, registered_keyboard) in registered_keyboards.into_iter().enumerate() {
        let part = library::read_as_vector(
            interpreter,
            registered_keyboard
        )?;

        let keyboard_name = library::read_as_string(
            interpreter,
            part[1]
        )?.clone();

        result.insert(keyboard_name, KeyboardId::new(index as u16));
    }

    Ok(result)
}

pub fn define_global_mapping(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `keyboard:define-global-mapping' takes two arguments exactly."
        ).into_result();
    }

    let mut values = values;

    let mapping = library::read_as_string(interpreter, values.remove(0))?.clone(); // todo: fix
    let function_id = library::read_as_function_id(interpreter, values.remove(0))?;

    let mut key_chords = Vec::new();
    let keyboard_name_to_index = read_registered_keyboards(interpreter)?;

    for mapping_part in mapping.split(" ") {
        let key_chord = nia_events::str_to_key_chord(
            mapping_part,
            &keyboard_name_to_index
        ).map_err(|_| Error::invalid_argument_error(
            "Invalid key chord."
        ))?;

        key_chords.push(key_chord);
    }

    let values = key_chords_to_list(interpreter, key_chords);

    let new_mapping = interpreter.vec_to_list(vec!(values, function_id.to_value()));
    library::add_value_to_root_list(
        interpreter,
        "global-map",
        new_mapping
    )?;

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
    fn defines_new_mappings_with_keyboard_identifiers() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("global-map", "'()"),
            ("(keyboard:define-global-mapping \"0:LeftControl+0:b\" #(+ 1 2))", "nil"),
            ("global-map", "(list (list (list (list (list 0 29) (list 0 48))) #(+ 1 2)))"),
            ("(keyboard:define-global-mapping \"1:LeftControl+1:c 1:LeftControl+1:b\" #())", "nil"),
            ("global-map", "(list (list (list (list (list 1 29) (list 1 46)) (list (list 1 29) (list 1 48))) #()) (list (list (list (list 0 29) (list 0 48))) #(+ 1 2)))"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn defines_new_mappings_with_keyboard_names() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(keyboard:register \"/dev/input/event0\" \"second\")").unwrap();
        interpreter.execute("(keyboard:register \"/dev/input/event1\" \"first\")").unwrap();

        let pairs = vec!(
            ("global-map", "'()"),
            ("(keyboard:define-global-mapping \"first:LeftControl+first:b\" #(+ 1 2))", "nil"),
            ("global-map", "(list (list (list (list (list 0 29) (list 0 48))) #(+ 1 2)))"),
            ("(keyboard:define-global-mapping \"second:LeftControl+second:c second:LeftControl+second:b\" #())", "nil"),
            ("global-map", "(list (list (list (list (list 1 29) (list 1 46)) (list (list 1 29) (list 1 48))) #()) (list (list (list (list 0 29) (list 0 48))) #(+ 1 2)))"),
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
