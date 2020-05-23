use std::collections::HashMap;

use crate::Error;
use crate::Interpreter;
use crate::{KeyChord, Mapping};

use crate::library;
use crate::GLOBAL_MAP_ROOT_VARIABLE_NAME;

pub fn remove_global_mapping(
    interpreter: &mut Interpreter,
    key_chord_sequence: &Vec<KeyChord>,
) -> Result<(), Error> {
    let mappings_value =
        library::get_root_variable(interpreter, GLOBAL_MAP_ROOT_VARIABLE_NAME)?;

    let mappings_vector = library::read_as_vector(interpreter, mappings_value)?;

    let mut result = Vec::new();

    for mapping_value in mappings_vector {
        let mapping = library::list_to_mapping(interpreter, mapping_value)?;

        if KeyChord::key_chord_vectors_are_same(
            mapping.get_key_chords(),
            key_chord_sequence,
        ) {
            continue;
        }

        result.push(mapping_value)
    }

    let result = interpreter.vec_to_list(result);

    library::set_root_variable(
        interpreter,
        GLOBAL_MAP_ROOT_VARIABLE_NAME,
        result,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::Action;
    use crate::Key;
    use crate::KeyChord;

    use crate::utils;

    #[test]
    fn removes_mapping_correctly() {
        let mut interpreter = Interpreter::new();

        let key_1 = nia_key!(1);
        let key_2 = nia_key!(2);
        let key_3 = nia_key!(3);
        let key_4 = nia_key!(4);

        let key_chord_1 = KeyChord::new(vec![key_1, key_2], key_4);

        let key_chord_2 = KeyChord::new(vec![key_1, key_3], key_4);

        let key_chord_3 = KeyChord::new(vec![key_2, key_3], key_4);

        let key_chord_sequence_1 =
            vec![key_chord_1.clone(), key_chord_2.clone()];
        let key_chord_sequence_2 = vec![key_chord_1, key_chord_3.clone()];
        let key_chord_sequence_3 = vec![key_chord_2, key_chord_3];

        let action_1 = Action::KeyClick(1);
        let action_2 = Action::KeyClick(2);
        let action_3 = Action::KeyClick(3);

        let mapping_1 =
            Mapping::new(key_chord_sequence_1.clone(), action_1.clone());
        let mapping_2 =
            Mapping::new(key_chord_sequence_2.clone(), action_2.clone());
        let mapping_3 =
            Mapping::new(key_chord_sequence_3.clone(), action_3.clone());

        let specs = vec![
            (
                key_chord_sequence_2,
                vec![mapping_3.clone(), mapping_1.clone()],
            ),
            (key_chord_sequence_1, vec![mapping_3.clone()]),
            (key_chord_sequence_3, vec![]),
        ];

        library::define_global_mapping(&mut interpreter, &mapping_1).unwrap();
        library::define_global_mapping(&mut interpreter, &mapping_2).unwrap();
        library::define_global_mapping(&mut interpreter, &mapping_3).unwrap();

        for (key_chord_sequence, expected_mappings) in specs {
            remove_global_mapping(&mut interpreter, &key_chord_sequence)
                .unwrap();

            let mappings =
                library::get_defined_mappings(&mut interpreter).unwrap();

            nia_assert_equal(expected_mappings.len(), mappings.len());

            for (expected_mapping, mapping) in
                expected_mappings.into_iter().zip(mappings.into_iter())
            {
                nia_assert(Mapping::mappings_are_same(
                    &expected_mapping,
                    &mapping,
                ));
            }
        }
    }
}
