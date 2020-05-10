use std::convert::TryInto;

use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

use nia_events::KeyChord;

pub fn read_as_key_chord(
    interpreter: &mut Interpreter,
    key_chord_value: Value,
) -> Result<KeyChord, Error> {
    library::check_value_is_cons(key_chord_value)?;

    let key_chord_cons_id = key_chord_value.try_into()?;
    let key_chord_part_values = interpreter.list_to_vec(key_chord_cons_id)?;
    let mut key_chord_parts = Vec::new();

    for key_chord_part_value in key_chord_part_values {
        let key_chord_part =
            library::read_as_key_chord_part(interpreter, key_chord_part_value)?;
        key_chord_parts.push(key_chord_part);
    }

    let (modifiers, key) = key_chord_parts.split_at(key_chord_parts.len() - 1);

    Ok(KeyChord::new(modifiers.to_vec(), key[0]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use nia_events::{KeyChordPart, KeyId, KeyboardId};

    fn assert_returns_correct_key_chord(s: &str, key_chord: KeyChord) {
        let mut interpreter = Interpreter::new();

        let expected = key_chord;

        let list = interpreter.execute_in_main_environment(s).unwrap();

        let result = read_as_key_chord(&mut interpreter, list).unwrap();

        nia_assert_equal(expected, result);
    }

    #[test]
    fn returns_correct_key_chord() {
        let specs = vec![
            (
                "'(0 2 4)",
                KeyChord::new(
                    vec![
                        KeyChordPart::Key1(KeyId::new(0)),
                        KeyChordPart::Key1(KeyId::new(2)),
                    ],
                    KeyChordPart::Key1(KeyId::new(4)),
                ),
            ),
            (
                "'(0 2 (4 5))",
                KeyChord::new(
                    vec![
                        KeyChordPart::Key1(KeyId::new(0)),
                        KeyChordPart::Key1(KeyId::new(2)),
                    ],
                    KeyChordPart::Key2(KeyboardId::new(4), KeyId::new(5)),
                ),
            ),
            (
                "'(0 (2 3) 4)",
                KeyChord::new(
                    vec![
                        KeyChordPart::Key1(KeyId::new(0)),
                        KeyChordPart::Key2(KeyboardId::new(2), KeyId::new(3)),
                    ],
                    KeyChordPart::Key1(KeyId::new(4)),
                ),
            ),
            (
                "'(0 (2 3) (4 5))",
                KeyChord::new(
                    vec![
                        KeyChordPart::Key1(KeyId::new(0)),
                        KeyChordPart::Key2(KeyboardId::new(2), KeyId::new(3)),
                    ],
                    KeyChordPart::Key2(KeyboardId::new(4), KeyId::new(5)),
                ),
            ),
            (
                "'((0 1) 2 4)",
                KeyChord::new(
                    vec![
                        KeyChordPart::Key2(KeyboardId::new(0), KeyId::new(1)),
                        KeyChordPart::Key1(KeyId::new(2)),
                    ],
                    KeyChordPart::Key1(KeyId::new(4)),
                ),
            ),
            (
                "'((0 1) 2 (4 5))",
                KeyChord::new(
                    vec![
                        KeyChordPart::Key2(KeyboardId::new(0), KeyId::new(1)),
                        KeyChordPart::Key1(KeyId::new(2)),
                    ],
                    KeyChordPart::Key2(KeyboardId::new(4), KeyId::new(5)),
                ),
            ),
            (
                "'((0 1) (2 3) 4)",
                KeyChord::new(
                    vec![
                        KeyChordPart::Key2(KeyboardId::new(0), KeyId::new(1)),
                        KeyChordPart::Key2(KeyboardId::new(2), KeyId::new(3)),
                    ],
                    KeyChordPart::Key1(KeyId::new(4)),
                ),
            ),
            (
                "'((0 1) (2 3) (4 5))",
                KeyChord::new(
                    vec![
                        KeyChordPart::Key2(KeyboardId::new(0), KeyId::new(1)),
                        KeyChordPart::Key2(KeyboardId::new(2), KeyId::new(3)),
                    ],
                    KeyChordPart::Key2(KeyboardId::new(4), KeyId::new(5)),
                ),
            ),
        ];

        for spec in specs {
            assert_returns_correct_key_chord(spec.0, spec.1);
        }
    }
}
