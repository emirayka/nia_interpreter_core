use crate::Error;
use crate::Interpreter;
use crate::ModifierDescription;
use crate::Value;

use crate::library;

pub fn get_defined_modifiers(
    interpreter: &mut Interpreter,
) -> Result<Vec<ModifierDescription>, Error> {
    let list = library::get_defined_modifiers_as_value(interpreter)?;

    let modifier_lists = library::read_as_vector(interpreter, list)?;

    let mut result = Vec::new();

    for modifier_list in modifier_lists {
        let modifier = library::read_as_vector(interpreter, modifier_list)?;

        let (key, modifier_alias_value) = match modifier.len() {
            2 => {
                let key_code = library::read_as_i64(modifier[0])? as i32;
                let modifier_alias_value = modifier[1];

                (nia_key!(key_code), modifier_alias_value)
            }
            3 => {
                let device_id = library::read_as_i64(modifier[0])? as i32;
                let key_code = library::read_as_i64(modifier[1])? as i32;
                let modifier_alias_value = modifier[2];

                (nia_key!(device_id, key_code), modifier_alias_value)
            }
            _ => {
                return Error::generic_execution_error(
                    "Invariant violation: `nia-defined-modifiers' must be a list of two or three element lists."
                ).into();
            }
        };

        let modifier_alias = match modifier_alias_value {
            Value::Symbol(symbol_id) => {
                if interpreter.symbol_is_nil(symbol_id)? {
                    String::from("")
                } else {
                    return Error::generic_execution_error(
                        "Invariant violation: modifier alias element of lists in `nia-defined-modifiers' must be a nil or string."
                    ).into();
                }
            }
            Value::String(string_id) => {
                interpreter.get_string(string_id)?
                    .get_string().clone()
            }
            _ => return Error::generic_execution_error(
                "Invariant violation: third element of lists in `nia-defined-modifiers' must be a nil or string."
            ).into()
        };

        result.push(ModifierDescription::new(key, modifier_alias));
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
    fn returns_defined_modifiers() {
        let mut interpreter = Interpreter::new();

        let result = get_defined_modifiers(&mut interpreter);
        let expected = Ok(Vec::new());

        nia_assert_equal(expected, result);

        let specs = vec![
            (
                nia_modifier!(3, 1, "test"),
                vec![nia_modifier!(3, 1, "test")],
            ),
            (
                nia_modifier!(2, ""),
                vec![nia_modifier!(2, ""), nia_modifier!(3, 1, "test")],
            ),
            (
                nia_modifier!(1, 3, "arst"),
                vec![
                    nia_modifier!(1, 3, "arst"),
                    nia_modifier!(2, ""),
                    nia_modifier!(3, 1, "test"),
                ],
            ),
        ];

        for spec in specs {
            let modifier_to_add = spec.0;

            library::define_modifier(&mut interpreter, &modifier_to_add)
                .unwrap();

            let expected = spec.1;
            let result = get_defined_modifiers(&mut interpreter).unwrap();

            nia_assert_equal(expected, result);
        }
    }
}
