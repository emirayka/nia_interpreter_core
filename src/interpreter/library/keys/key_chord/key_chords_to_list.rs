use crate::Interpreter;
use crate::KeyChord;
use crate::Value;

use crate::library;

pub fn key_chords_to_list(
    interpreter: &mut Interpreter,
    key_chords: &Vec<KeyChord>,
) -> Value {
    let mut vector = Vec::new();

    for key_chord in key_chords {
        vector.push(library::key_chord_to_list(interpreter, key_chord));
    }

    interpreter.vec_to_list(vector)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::utils;

    fn assert_returns_correct_list(s: &str, key_chords: Vec<KeyChord>) {
        let mut interpreter = Interpreter::new();

        let expected = interpreter.execute_in_main_environment(s).unwrap();
        let result = key_chords_to_list(&mut interpreter, &key_chords);

        utils::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn returns_correct_list_of_key_chords() {
        let specs = vec![
            (
                "'((0 1 2))",
                vec![KeyChord::new(
                    vec![nia_key!(0), nia_key!(1)],
                    nia_key!(2),
                )],
            ),
            (
                "'((0 1 2) (3 4 5))",
                vec![
                    KeyChord::new(vec![nia_key!(0), nia_key!(1)], nia_key!(2)),
                    KeyChord::new(vec![nia_key!(3), nia_key!(4)], nia_key!(5)),
                ],
            ),
            (
                "'((0 1 2) (3 4 5) ((0 1) (1 1) (1 2)))",
                vec![
                    KeyChord::new(vec![nia_key!(0), nia_key!(1)], nia_key!(2)),
                    KeyChord::new(vec![nia_key!(3), nia_key!(4)], nia_key!(5)),
                    KeyChord::new(
                        vec![nia_key!(0, 1), nia_key!(1, 1)],
                        nia_key!(1, 2),
                    ),
                ],
            ),
        ];

        for (expected, key_chords) in specs {
            assert_returns_correct_list(expected, key_chords)
        }
    }
}
