use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use nia_events::KeyChordPart;

pub fn key_chord_part_to_list(
    interpreter: &mut Interpreter,
    key_chord_part: KeyChordPart,
) -> Value {
    match key_chord_part {
        KeyChordPart::Key1(key_id) => Value::Integer(key_id.get_id() as i64),
        KeyChordPart::Key2(keyboard_id, key_id) => {
            interpreter.vec_to_list(vec![
                Value::Integer(keyboard_id.get_id() as i64),
                Value::Integer(key_id.get_id() as i64),
            ])
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;
    use nia_events::{KeyId, KeyboardId};

    fn assert_returns_correct_list(s: &str, key_chord_part: KeyChordPart) {
        let mut interpreter = Interpreter::new();
        let expected = interpreter.execute_in_main_environment(s).unwrap();
        let result = key_chord_part_to_list(&mut interpreter, key_chord_part);

        utils::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn returns_correct_key_chord_part_list_representations() {
        let specs = vec![
            ("0", KeyChordPart::Key1(KeyId::new(0))),
            (
                "'(0 1)",
                KeyChordPart::Key2(KeyboardId::new(0), KeyId::new(1)),
            ),
        ];

        for spec in specs {
            assert_returns_correct_list(spec.0, spec.1);
        }
    }
}
