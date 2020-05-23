use std::collections::HashMap;

use crate::Action;
use crate::Error;
use crate::Interpreter;
use crate::Key;
use crate::KeyChord;
use crate::Mapping;

use crate::library;
use crate::GLOBAL_MAP_ROOT_VARIABLE_NAME;

pub fn change_global_mapping(
    interpreter: &mut Interpreter,
    key_chord_sequence: &Vec<KeyChord>,
    action: &Action,
) -> Result<(), Error> {
    let mappings_value =
        library::get_root_variable(interpreter, GLOBAL_MAP_ROOT_VARIABLE_NAME)?;
    let mappings_values = library::read_as_vector(interpreter, mappings_value)?;

    for mapping_value in mappings_values {
        let mapping_cons_id = library::read_as_cons_id(mapping_value)?;
        let mapping = library::list_to_mapping(interpreter, mapping_value)?;
        let mapping_key_chords = mapping.get_key_chords();

        if KeyChord::key_chord_vectors_are_same(
            mapping_key_chords,
            key_chord_sequence,
        ) {
            let action_list = library::action_to_list(interpreter, action)?;

            interpreter.set_cdr(mapping_cons_id, action_list);

            return Ok(());
        }
    }

    Error::generic_execution_error("Mapping was not found").into()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn changes_mapping() {
        let mut interpreter = Interpreter::new();

        let key_chords = vec![
            KeyChord::new(vec![nia_key!(1)], nia_key!(2)),
            KeyChord::new(vec![nia_key!(1)], nia_key!(3)),
        ];
        let mapping = Mapping::new(key_chords.clone(), Action::Wait(1000));

        library::define_global_mapping(&mut interpreter, &mapping).unwrap();

        let expected = interpreter
            .execute_in_main_environment(
                "(list:new (list:new '((1 2) (1 3)) 'wait 1000))",
            )
            .unwrap();
        let result = library::get_root_variable(
            &mut interpreter,
            GLOBAL_MAP_ROOT_VARIABLE_NAME,
        )
        .unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result);

        let new_action = Action::ExecuteOSCommand(String::from("echo cat"));
        change_global_mapping(&mut interpreter, &key_chords, &new_action)
            .unwrap();

        let expected = interpreter
            .execute_in_main_environment(
                "(list:new (list:new '((1 2) (1 3)) 'execute-os-command \"echo cat\"))",
            )
            .unwrap();
        let result = library::get_root_variable(
            &mut interpreter,
            GLOBAL_MAP_ROOT_VARIABLE_NAME,
        )
        .unwrap();
        crate::utils::assert_deep_equal(&mut interpreter, expected, result);
    }
}
