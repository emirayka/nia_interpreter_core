use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use super::key_chord_part_to_list;

use nia_events::KeyChord;

pub fn key_chord_to_list(
    interpreter: &mut Interpreter,
    key_chord: KeyChord,
) -> Value {
    let mut vector = Vec::new();

    for modifier in key_chord.get_modifiers() {
        vector.push(key_chord_part_to_list(interpreter, *modifier));
    }

    vector.push(key_chord_part_to_list(interpreter, *key_chord.get_key()));

    interpreter.vec_to_list(vector)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    use nia_events::{KeyChordPart, KeyId, KeyboardId};

    fn assert_returns_correct_list(s: &str, key_chord: KeyChord) {
        let mut interpreter = Interpreter::new();

        let expected = interpreter.execute_in_main_environment(s).unwrap();

        let result = key_chord_to_list(&mut interpreter, key_chord);

        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn constructs_correct_list_from_key_chord() {
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
            assert_returns_correct_list(spec.0, spec.1);
        }
    }
}
