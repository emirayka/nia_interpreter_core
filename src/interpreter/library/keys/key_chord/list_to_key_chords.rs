use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::Error;
use crate::KeyChord;

use crate::library;

pub fn list_to_key_chords(
    interpreter: &mut Interpreter,
    key_chords_list: Value,
) -> Result<Vec<KeyChord>, Error> {
    let key_chords_value_vector =
        library::read_as_vector(interpreter, key_chords_list)?;

    let mut key_chords = Vec::new();

    for key_chord_value in key_chords_value_vector {
        let key_chord =
            library::list_to_key_chord(interpreter, key_chord_value)?;

        key_chords.push(key_chord);
    }

    Ok(key_chords)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn parses_key_chord_correctly() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (vec![], "'()"),
            (vec![KeyChord::new(vec![], nia_key!(1))], "'((1))"),
            (
                vec![
                    KeyChord::new(vec![], nia_key!(1)),
                    KeyChord::new(vec![], nia_key!(2)),
                ],
                "'((1) (2))",
            ),
            (
                vec![
                    KeyChord::new(vec![], nia_key!(1)),
                    KeyChord::new(vec![], nia_key!(2)),
                    KeyChord::new(vec![], nia_key!(3)),
                ],
                "'((1) (2) (3))",
            ),
        ];

        for (expected, code) in specs {
            let value = interpreter.execute_in_main_environment(code).unwrap();
            let result = list_to_key_chords(&mut interpreter, value).unwrap();

            // todo: probably more sophisticated testing
            nia_assert_equal(expected, result);
        }
    }
}
