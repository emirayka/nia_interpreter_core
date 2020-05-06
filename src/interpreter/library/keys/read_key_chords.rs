use std::convert::TryInto;

use nia_events::KeyChord;

use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn read_key_chords(
    interpreter: &mut Interpreter,
    key_chords_value: Value,
) -> Result<Vec<KeyChord>, Error> {
    library::check_value_is_cons(interpreter, key_chords_value)?;

    let key_chords_cons_id = key_chords_value.try_into()?;
    let key_chords_values = interpreter.list_to_vec(key_chords_cons_id)?;
    let mut key_chords = Vec::new();

    for key_chord_value in key_chords_values {
        let key_chord =
            library::read_as_key_chord(interpreter, key_chord_value)?;

        key_chords.push(key_chord);
    }

    Ok(key_chords)
}
