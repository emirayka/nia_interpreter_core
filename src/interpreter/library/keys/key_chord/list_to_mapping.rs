use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::Error;
use crate::Mapping;

use crate::library;

pub fn list_to_mapping(
    interpreter: &mut Interpreter,
    mapping_list: Value,
) -> Result<Mapping, Error> {
    let mapping_cons_id = library::read_as_cons_id(mapping_list)?;

    let key_chords_value = interpreter.get_car(mapping_cons_id)?;
    let action_value = interpreter.get_cdr(mapping_cons_id)?;

    let key_chords =
        library::list_to_key_chords(interpreter, key_chords_value)?;

    let action = library::list_to_action(interpreter, action_value)?;

    let mapping = Mapping::new(key_chords, action);

    Ok(mapping)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    use crate::Action;
    use crate::KeyChord;

    #[test]
    fn reads_mappings_correctly() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                Mapping::new(
                    vec![KeyChord::new(vec![], nia_key!(1))],
                    Action::KeyRelease(10),
                ),
                "(cons:new '((1)) '(key-release 10))",
            ),
            (
                Mapping::new(
                    vec![KeyChord::new(
                        vec![nia_key!(1, 2), nia_key!(1, 3)],
                        nia_key!(1, 4),
                    )],
                    Action::KeyClick(21),
                ),
                "(cons:new '(((1 2) (1 3) (1 4))) '(key-click 21))",
            ),
        ];

        for (expected, code) in specs {
            let value = interpreter.execute_in_main_environment(code).unwrap();
            let result = list_to_mapping(&mut interpreter, value).unwrap();

            nia_assert_equal(expected, result);
        }
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_argument_were_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "1",
            "1.1",
            "#t",
            "#f",
            "\"string\"",
            ":keyword",
            "'symbol",
            "'(1 2 3)",
            "{}",
            "#()",
        ];

        for spec in specs {
            let value = interpreter.execute_in_main_environment(spec).unwrap();
            let result = list_to_mapping(&mut interpreter, value);

            utils::assert_invalid_argument_error(&result);
        }
    }
}
