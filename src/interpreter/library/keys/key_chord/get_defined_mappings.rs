use std::collections::HashMap;

use crate::{Error, GLOBAL_MAP_ROOT_VARIABLE_NAME};
use crate::{Interpreter, Mapping};

use crate::library;

pub fn get_defined_mappings(
    interpreter: &mut Interpreter,
) -> Result<Vec<Mapping>, Error> {
    let mappings_value =
        library::get_root_variable(interpreter, GLOBAL_MAP_ROOT_VARIABLE_NAME)?;

    let mappings_vector = library::read_as_vector(interpreter, mappings_value)?;
    let mut result = Vec::new();

    for mapping_value in mappings_vector {
        let mapping = library::list_to_mapping(interpreter, mapping_value)?;

        result.push(mapping);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::Action;
    use crate::Key;
    use crate::KeyChord;
    use nia_basic_assertions::nia_assert_equal;

    #[test]
    fn returns_list_of_mappings() {
        let mut interpreter = Interpreter::new();

        let expected_mappings = vec![
            Mapping::new(
                vec![
                    KeyChord::new(
                        vec![
                            nia_key!(1, 2),
                            nia_key!(1, 3),
                        ],
                        nia_key!(1, 4),
                    ),
                    KeyChord::new(
                        vec![
                            nia_key!(2, 2),
                            nia_key!(2, 3),
                        ],
                        nia_key!(2, 4),
                    ),
                ],
                Action::KeyPress(10),
            ),
            Mapping::new(
                vec![KeyChord::new(vec![], nia_key!(2, 5))],
                Action::KeyRelease(10),
            ),
        ];

        for mapping in expected_mappings.iter().rev() {
            library::define_global_mapping(&mut interpreter, mapping).unwrap();
        }

        let result = get_defined_mappings(&mut interpreter).unwrap();

        nia_assert_equal(expected_mappings, result);
    }
}
