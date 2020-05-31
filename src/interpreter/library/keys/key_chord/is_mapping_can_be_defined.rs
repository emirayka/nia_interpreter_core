use crate::Error;
use crate::Interpreter;
use crate::KeyChord;

use crate::library;

fn is_prefix_of_another(
    key_chords: &Vec<KeyChord>,
    another: &Vec<KeyChord>,
) -> bool {
    if key_chords.len() > another.len() {
        return false;
    }

    let mut key_chords_iter = key_chords.iter();
    let mut another_iter = another.iter();

    while let Some(another_key_chord) = another_iter.next() {
        match key_chords_iter.next() {
            Some(key_chord) => {
                if !KeyChord::key_chords_are_same(key_chord, another_key_chord)
                {
                    return false;
                }
            }
            None => break,
        };
    }

    true
}

pub fn is_mapping_can_be_defined(
    interpreter: &mut Interpreter,
    key_chords: &Vec<KeyChord>,
) -> Result<bool, Error> {
    let defined_mappings = library::get_defined_mappings(interpreter)?;

    for defined_mapping in defined_mappings {
        if is_prefix_of_another(defined_mapping.get_key_chords(), key_chords) {
            return Ok(false);
        }

        if is_prefix_of_another(key_chords, defined_mapping.get_key_chords()) {
            return Ok(false);
        }
    }

    Ok(true)
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

        let result = library::is_mapping_can_be_defined(
            &mut interpreter,
            mapping.get_key_chords(),
        )
        .unwrap();
        nia_assert(result);

        nia_assert_is_ok(&library::define_global_mapping(
            &mut interpreter,
            &mapping,
        ));

        let result = library::is_mapping_can_be_defined(
            &mut interpreter,
            mapping.get_key_chords(),
        )
        .unwrap();
        nia_assert(!result);
    }
}
