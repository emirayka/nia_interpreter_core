use nia_events::KeyChord;

use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn read_mapping(
    interpreter: &mut Interpreter,
    mapping_value: Value,
) -> Result<(Vec<KeyChord>, Value), Error> {
    let values = library::read_as_vector(interpreter, mapping_value)?;
    let key_chords = library::read_key_chords(interpreter, values[0])?;
    let function = values[1];

    Ok((key_chords, function))
}
