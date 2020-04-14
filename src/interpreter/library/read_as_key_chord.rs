use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

use nia_events::{KeyChordPart, KeyId, KeyboardId, KeyChord};

pub fn read_as_key_chord(
    interpreter: &mut Interpreter,
    key_chord_value: Value
) -> Result<KeyChord, Error> {
    library::check_value_is_cons(interpreter, key_chord_value)?;

    let key_chord_part_values = interpreter.list_to_vec(key_chord_value.as_cons_id())?;
    let mut key_chord_parts = Vec::new();

    for key_chord_part_value in key_chord_part_values {
        let key_chord_part = library::read_as_key_chord_part(interpreter, key_chord_part_value)?;
        key_chord_parts.push(key_chord_part);
    }

    let (modifiers, key) = key_chord_parts.split_at(
        key_chord_parts.len() - 1
    );

    Ok(KeyChord::new(
        modifiers.to_vec(),
        key[0]
    ))
}

// todo: tests
