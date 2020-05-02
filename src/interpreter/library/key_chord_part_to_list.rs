use crate::interpreter::value::Value;
use crate::interpreter::interpreter::Interpreter;

use nia_events::KeyChordPart;

pub fn key_chord_part_to_list(
    interpreter: &mut Interpreter,
    key_chord_part: KeyChordPart,
) -> Value {
    match key_chord_part {
        KeyChordPart::Key1(key_id) => {
            Value::Integer(key_id.get_id() as i64)
        }
        KeyChordPart::Key2(keyboard_id, key_id) => {
            interpreter.vec_to_list(
                vec!(
                    Value::Integer(keyboard_id.get_id() as i64),
                    Value::Integer(key_id.get_id() as i64)
                )
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use nia_events::{KeyId, KeyboardId};
    use crate::interpreter::library::assertion;

    fn assert_returns_correct_list(s: &str, key_chord_part: KeyChordPart) {
        let mut interpreter = Interpreter::new();

        let expected = interpreter.execute(
            s
        ).unwrap();

        let result = key_chord_part_to_list(
            &mut interpreter,
            key_chord_part,
        );

        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result,
        );
    }

    #[test]
    fn returns_correct_key_chord_part_list_representations() {
        let specs = vec!(
            (
                "0",
                KeyChordPart::Key1(KeyId::new(0))
            ),
            (
                "'(0 1)",
                KeyChordPart::Key2(KeyboardId::new(0), KeyId::new(1))
            )
        );

        for spec in specs {
            assert_returns_correct_list(spec.0, spec.1);
        }
    }
}

