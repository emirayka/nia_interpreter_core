use crate::Error;
use crate::Interpreter;
use crate::KeyChord;

use crate::library;

pub fn is_mapping_defined(
    interpreter: &mut Interpreter,
    key_chords: &Vec<KeyChord>,
) -> Result<bool, Error> {
    let defined_mappings = library::get_defined_mappings(interpreter)?;

    for defined_mapping in defined_mappings {
        if KeyChord::key_chord_vectors_are_same(
            defined_mapping.get_key_chords(),
            key_chords,
        ) {
            return Ok(true);
        }
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::Action;
    use crate::Mapping;

    #[test]
    fn works_correctly() {
        let mut interpreter = Interpreter::new();

        let key_chords = vec![
            KeyChord::new(vec![nia_key!(1)], nia_key!(4)),
            KeyChord::new(vec![nia_key!(2)], nia_key!(4)),
        ];

        let mapping = Mapping::new(key_chords, Action::KeyClick(1));

        let result = library::is_mapping_defined(
            &mut interpreter,
            mapping.get_key_chords(),
        )
        .unwrap();
        nia_assert(!result);

        nia_assert_is_ok(&library::define_global_mapping(
            &mut interpreter,
            &mapping,
        ));

        let result = library::is_mapping_defined(
            &mut interpreter,
            mapping.get_key_chords(),
        )
        .unwrap();
        nia_assert(result);
    }
}
