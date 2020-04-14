use crate::interpreter::value::Value;
use crate::interpreter::interpreter::Interpreter;

use super::key_chord_part_to_list;

use nia_events::{KeyChordPart, KeyChord};

pub fn key_chord_to_list(
    interpreter: &mut Interpreter,
    key_chord: KeyChord
) -> Value {
    let mut vector = Vec::new();

    for modifier in key_chord.get_modifiers() {
        vector.push(key_chord_part_to_list(interpreter, *modifier));
    }

    vector.push(key_chord_part_to_list(interpreter, *key_chord.get_key()));

    interpreter.vec_to_list(vector)
}

// todo: tests
