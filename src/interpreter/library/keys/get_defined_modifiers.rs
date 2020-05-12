use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn get_defined_modifiers(
    interpreter: &mut Interpreter,
) -> Result<Vec<(String, i32, String)>, Error> {
    let list = library::get_defined_modifiers_as_values(interpreter)?;

    let modifier_lists = library::read_as_vector(interpreter, list)?;

    let mut result = Vec::new();

    for modifier_list in modifier_lists {
        let modifier = library::read_as_vector(interpreter, modifier_list)?;

        if modifier.len() != 3 {
            return Error::generic_execution_error(
                "Invariant violation: `nia-defined-modifiers' must be a list of three-element lists."
            ).into();
        }

        let keyboard_path =
            library::read_as_string(interpreter, modifier[0])?.clone();

        let key_code = library::read_as_i64(modifier[1])? as i32;

        let modifier_alias = match modifier[2] {
            Value::Symbol(symbol_id) => {
                if interpreter.symbol_is_nil(symbol_id)? {
                    String::from("")
                } else {
                    return Error::generic_execution_error(
                        "Invariant violation: third element of lists in `nia-defined-modifiers' must be a nil or string."
                    ).into()
                }
            },
            Value::String(string_id) => {
                interpreter.get_string(string_id)?
                    .get_string().clone()
            },
            _ => return Error::generic_execution_error(
                "Invariant violation: third element of lists in `nia-defined-modifiers' must be a nil or string."
            ).into()
        };

        result.push((keyboard_path, key_code, modifier_alias));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn returns_registered_keyboards() {
        let mut interpreter = Interpreter::new();

        let result = get_defined_modifiers(&mut interpreter);
        let expected = Ok(Vec::new());

        nia_assert_equal(expected, result);

        let specs = vec![
            (
                ("/dev/input/event1", 1, "test"),
                Ok(vec![(
                    "/dev/input/event1".to_string(),
                    1,
                    "test".to_string(),
                )]),
            ),
            (
                ("/dev/input/event2", 2, ""),
                Ok(vec![
                    ("/dev/input/event2".to_string(), 2, "".to_string()),
                    ("/dev/input/event1".to_string(), 1, "test".to_string()),
                ]),
            ),
            (
                ("/dev/input/event3", 3, "arst"),
                Ok(vec![
                    ("/dev/input/event3".to_string(), 3, "arst".to_string()),
                    ("/dev/input/event2".to_string(), 2, "".to_string()),
                    ("/dev/input/event1".to_string(), 1, "test".to_string()),
                ]),
            ),
        ];

        for spec in specs {
            let modifier_to_add = spec.0;
            library::define_modifier(
                &mut interpreter,
                modifier_to_add.0,
                modifier_to_add.1,
                modifier_to_add.2,
            )
            .unwrap();

            let expected = spec.1;
            let result = get_defined_modifiers(&mut interpreter);
            println!("{:?}", expected);
            println!("{:?}", result);

            nia_assert_equal(expected, result);
        }
    }
}
