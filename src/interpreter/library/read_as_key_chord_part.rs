use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

use nia_events::{KeyChordPart, KeyboardId, KeyId};

pub fn read_as_key_chord_part(
    interpreter: &mut Interpreter,
    key_chord_part_value: Value
) -> Result<KeyChordPart, Error> {
    match key_chord_part_value {
        Value::Integer(key_id) => Ok(KeyChordPart::Key1(KeyId::new(key_id as u16))),
        Value::Cons(cons_id) => {
            let mut values = interpreter.list_to_vec(cons_id)?;

            let keyboard_id = library::read_as_i64(interpreter, values.remove(0))?;
            let key_id = library::read_as_i64(interpreter, values.remove(0))?;

            Ok(KeyChordPart::Key2(
                KeyboardId::new(keyboard_id as u16),
                KeyId::new(key_id as u16),
            ))
        },
        _ => interpreter.make_invalid_argument_error(
            "Invalid key chord part"
        ).into_result()
    }
}
