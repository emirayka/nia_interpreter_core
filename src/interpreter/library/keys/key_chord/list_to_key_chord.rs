use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::Error;
use crate::KeyChord;

use crate::library;

pub fn list_to_key_chord(
    interpreter: &mut Interpreter,
    key_chord_list: Value,
) -> Result<KeyChord, Error> {
    let key_chord_vector =
        library::read_as_vector(interpreter, key_chord_list)?;

    if key_chord_vector.len() == 0 {
        return Error::invalid_argument_error(
            "List must have one item at least to be considered as key chord",
        )
        .into();
    }

    let mut keys = Vec::new();

    for key_list in key_chord_vector {
        let key = library::list_to_key(interpreter, key_list)?;

        keys.push(key);
    }

    let key = keys.remove(keys.len() - 1);
    let modifiers = keys;

    Ok(KeyChord::new(modifiers, key))
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
            (KeyChord::new(vec![], nia_key!(1)), "'(1)"),
            (KeyChord::new(vec![], nia_key!(1, 2)), "'((1 2))"),
            (KeyChord::new(vec![nia_key!(1)], nia_key!(3)), "'(1 3)"),
            (
                KeyChord::new(vec![nia_key!(1)], nia_key!(3, 4)),
                "'(1 (3 4))",
            ),
            (
                KeyChord::new(vec![nia_key!(1, 2)], nia_key!(3)),
                "'((1 2) 3)",
            ),
            (
                KeyChord::new(vec![nia_key!(1, 2)], nia_key!(3, 4)),
                "'((1 2) (3 4))",
            ),
        ];

        for (expected, code) in specs {
            let key_chord_list =
                interpreter.execute_in_main_environment(code).unwrap();
            let result =
                list_to_key_chord(&mut interpreter, key_chord_list).unwrap();

            let expected_modifiers = expected.get_modifiers();
            let expected_key = expected.get_key();
            let result_modifiers = result.get_modifiers();
            let result_key = result.get_key();

            nia_assert_equal(
                expected_key.get_key_id(),
                result_key.get_key_id(),
            );
            nia_assert_equal(
                expected_key.get_device_id(),
                result_key.get_device_id(),
            );
            nia_assert_equal(expected_modifiers.len(), result_modifiers.len());

            for (expected_modifier, result_modifier) in
                expected_modifiers.iter().zip(result_modifiers.iter())
            {
                nia_assert_equal(
                    expected_modifier.get_key_id(),
                    result_modifier.get_key_id(),
                );
                nia_assert_equal(
                    expected_modifier.get_device_id(),
                    result_modifier.get_device_id(),
                );
            }
        }
    }

    #[test]
    fn returns_invalid_error_when_invalid_argument_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "1",
            "1.1",
            "#f",
            "#t",
            "\"string\"",
            ":keyword",
            "'symbol",
            "'()",
            "{}",
            "#()",
        ];

        for spec in specs {
            let value = interpreter.execute_in_main_environment(spec).unwrap();
            let result = list_to_key_chord(&mut interpreter, value);

            utils::assert_invalid_argument_error(&result);
        }
    }
}
