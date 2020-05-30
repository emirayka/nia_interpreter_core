use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::library;
use crate::KeyChord;

pub fn key_chord_to_list(
    interpreter: &mut Interpreter,
    key_chord: &KeyChord,
) -> Value {
    let mut vector = Vec::new();

    for modifier in key_chord.get_modifiers() {
        let list = library::key_to_list(interpreter, *modifier);
        vector.push(list);
    }

    let list = library::key_to_list(interpreter, key_chord.get_key());
    vector.push(list);

    interpreter.vec_to_list(vector)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    use crate::KeyChord;

    fn assert_returns_correct_list(s: &str, key_chord: KeyChord) {
        let mut interpreter = Interpreter::new();

        let expected = interpreter.execute_in_main_environment(s).unwrap();

        let result = key_chord_to_list(&mut interpreter, &key_chord);

        utils::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn constructs_correct_list_from_key_chord() {
        let specs = vec![
            (
                "'(0 2 4)",
                KeyChord::new(vec![nia_key!(0), nia_key!(2)], nia_key!(4)),
            ),
            (
                "'(0 2 (4 5))",
                KeyChord::new(vec![nia_key!(0), nia_key!(2)], nia_key!(4, 5)),
            ),
            (
                "'(0 (2 3) 4)",
                KeyChord::new(vec![nia_key!(0), nia_key!(2, 3)], nia_key!(4)),
            ),
            (
                "'(0 (2 3) (4 5))",
                KeyChord::new(
                    vec![nia_key!(0), nia_key!(2, 3)],
                    nia_key!(4, 5),
                ),
            ),
            (
                "'((0 1) 2 4)",
                KeyChord::new(vec![nia_key!(0, 1), nia_key!(2)], nia_key!(4)),
            ),
            (
                "'((0 1) 2 (4 5))",
                KeyChord::new(
                    vec![nia_key!(0, 1), nia_key!(2)],
                    nia_key!(4, 5),
                ),
            ),
            (
                "'((0 1) (2 3) 4)",
                KeyChord::new(
                    vec![nia_key!(0, 1), nia_key!(2, 3)],
                    nia_key!(4),
                ),
            ),
            (
                "'((0 1) (2 3) (4 5))",
                KeyChord::new(
                    vec![nia_key!(0, 1), nia_key!(2, 3)],
                    nia_key!(4, 5),
                ),
            ),
        ];

        for spec in specs {
            assert_returns_correct_list(spec.0, spec.1);
        }
    }
}
