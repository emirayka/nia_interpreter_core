use crate::interpreter::value::Value;
use crate::interpreter::interpreter::Interpreter;

use nia_events::KeyChordPart;

pub fn key_chord_part_to_list(
    interpreter: &mut Interpreter,
    key_chord_part: KeyChordPart
) -> Value {
    match key_chord_part {
        KeyChordPart::Key1(key_id) => {
            Value::Integer(key_id.get_id() as i64)
        },
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

// todo: tests
