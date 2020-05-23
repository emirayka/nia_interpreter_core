use crate::Interpreter;
use crate::Value;
use crate::{Error, DEFINED_MODIFIERS_ROOT_VARIABLE_NAME};

use crate::library;

fn check_modifier_can_be_defined(
    interpreter: &mut Interpreter,
    device_id: Option<Value>,
    key_code: Value,
) -> Result<(), Error> {
    let modifiers_list = library::get_defined_modifiers_as_value(interpreter)?;

    let modifiers_vector =
        library::read_as_vector(interpreter, modifiers_list)?;

    for modifier_list in modifiers_vector {
        let modifier_vector =
            library::read_as_vector(interpreter, modifier_list)?;

        let (modifier_device_id, modifier_key_code) = match modifier_vector
            .len()
        {
            2 => (None, modifier_vector[0]),
            3 => (Some(modifier_vector[0]), modifier_vector[1]),
            _ => {
                return Error::generic_execution_error(
                    "Invariant violation: `nia-defined-modifiers' must be a list of three-element lists."
                ).into();
            }
        };

        if modifier_device_id != device_id {
            continue;
        }

        if modifier_key_code != key_code {
            continue;
        }

        return Error::generic_execution_error("Modifier was already defined.")
            .into();
    }

    Ok(())
}

pub fn define_modifier_with_values(
    interpreter: &mut Interpreter,
    device_id: Option<Value>,
    key_code: Value,
    modifier_alias: Value,
) -> Result<(), Error> {
    if let Some(device_id) = device_id {
        library::check_value_is_integer(device_id)?;
    }

    library::check_value_is_integer(key_code)?;
    library::check_value_is_string_or_nil(interpreter, modifier_alias)?;

    check_modifier_can_be_defined(interpreter, device_id, key_code)?;

    let new_modifier_list = match device_id {
        Some(device_id) => {
            interpreter.vec_to_list(vec![device_id, key_code, modifier_alias])
        }
        None => interpreter.vec_to_list(vec![key_code, modifier_alias]),
    };

    library::add_value_to_root_list(
        interpreter,
        DEFINED_MODIFIERS_ROOT_VARIABLE_NAME,
        new_modifier_list,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn defines_new_modifiers() {
        let mut interpreter = Interpreter::new();

        let result =
            library::get_defined_modifiers_as_value(&mut interpreter).unwrap();
        let expected =
            interpreter.execute_in_main_environment(r#"'()"#).unwrap();
        crate::utils::assert_deep_equal(&mut interpreter, expected, result);

        let specs = vec![
            (
                Some(Value::Integer(3)),
                Value::Integer(1),
                interpreter.intern_nil_symbol_value(),
                r#"'((3 1 ()))"#,
            ),
            (
                None,
                Value::Integer(2),
                interpreter.intern_string_value("bb"),
                r#"'((2 "bb") (3 1 ()))"#,
            ),
            (
                Some(Value::Integer(1)),
                Value::Integer(3),
                interpreter.intern_string_value("cc"),
                r#"'((1 3 "cc") (2 "bb") (3 1 ()))"#,
            ),
        ];

        for (device_id, key_code, modifier_alias, code) in specs {
            nia_assert_is_ok(&define_modifier_with_values(
                &mut interpreter,
                device_id,
                key_code,
                modifier_alias,
            ));

            let expected =
                interpreter.execute_in_main_environment(code).unwrap();
            let result =
                library::get_defined_modifiers_as_value(&mut interpreter)
                    .unwrap();

            crate::utils::assert_deep_equal(&mut interpreter, expected, result)
        }
    }

    #[test]
    fn returns_generic_execution_error_when_attempts_to_define_already_defined_modifier(
    ) {
        let mut interpreter = Interpreter::new();

        let device_id = Some(Value::Integer(1));
        let key_code = Value::Integer(23);
        let modifier_alias = interpreter.intern_string_value("mod");

        nia_assert_is_ok(&define_modifier_with_values(
            &mut interpreter,
            device_id,
            key_code,
            modifier_alias,
        ));

        let result = &define_modifier_with_values(
            &mut interpreter,
            device_id,
            key_code,
            modifier_alias,
        );

        crate::utils::assert_generic_execution_error(&result);
    }
}
