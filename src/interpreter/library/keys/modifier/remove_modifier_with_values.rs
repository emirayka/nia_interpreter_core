use crate::Interpreter;
use crate::Value;
use crate::{Error, DEFINED_MODIFIERS_ROOT_VARIABLE_NAME};

use crate::library;

fn find_index(
    interpreter: &mut Interpreter,
    modifiers_vector: &Vec<Value>,
    device_id_value: Option<Value>,
    key_code_value: Value,
) -> Result<usize, Error> {
    let mut index = 0;

    for modifier_list in modifiers_vector {
        let modifier_vector =
            library::read_as_vector(interpreter, *modifier_list)?;

        let (modifier_device_id_value, modifier_key_code_value) =
            match modifier_vector.len() {
                2 => (None, modifier_vector[0]),
                3 => (Some(modifier_vector[0]), modifier_vector[1]),
                _ => {
                    return Error::generic_execution_error(
                    "Invariant violation: `nia-defined-modifiers' must be a list of two or three element lists."
                ).into();
                }
            };

        if modifier_device_id_value == device_id_value
            && modifier_key_code_value == key_code_value
        {
            break;
        }

        index += 1;
    }

    Ok(index)
}

pub fn remove_modifier_with_values(
    interpreter: &mut Interpreter,
    device_id_value: Option<Value>,
    key_code_value: Value,
) -> Result<(), Error> {
    if let Some(device_id) = device_id_value {
        library::check_value_is_integer(device_id)?;
    }
    library::check_value_is_integer(key_code_value)?;

    let modifiers_list = library::get_root_variable(
        interpreter,
        DEFINED_MODIFIERS_ROOT_VARIABLE_NAME,
    )?;
    let mut modifiers_vector =
        library::read_as_vector(interpreter, modifiers_list)?;

    let index = find_index(
        interpreter,
        &modifiers_vector,
        device_id_value,
        key_code_value,
    )?;

    if index < modifiers_vector.len() {
        modifiers_vector.remove(index);

        let new_list = interpreter.vec_to_list(modifiers_vector);

        library::set_root_variable(
            interpreter,
            DEFINED_MODIFIERS_ROOT_VARIABLE_NAME,
            new_list,
        )?;

        Ok(())
    } else {
        Error::generic_execution_error("Cannot find modifier.").into()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn removes_defined_modifiers() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (Some(Value::Integer(3)), Value::Integer(1)),
            (None, Value::Integer(2)),
            (Some(Value::Integer(1)), Value::Integer(3)),
        ];
        let modifier_alias_value = interpreter.intern_nil_symbol_value();

        for (device_id_value, key_code_value) in specs {
            nia_assert_is_ok(&library::define_modifier_with_values(
                &mut interpreter,
                device_id_value,
                key_code_value,
                modifier_alias_value,
            ));
        }

        let expected = interpreter
            .execute_in_main_environment(r#"'((1 3 ()) (2 ()) (3 1 ()))"#)
            .unwrap();
        let result =
            library::get_defined_modifiers_as_value(&mut interpreter).unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result);

        let specs = vec![
            (None, Value::Integer(2), r#"'((1 3 ()) (3 1 ()))"#),
            (Some(Value::Integer(1)), Value::Integer(3), r#"'((3 1 ()))"#),
            (Some(Value::Integer(3)), Value::Integer(1), r#"'()"#),
        ];

        for (device_id_value, key_code_value, code) in specs {
            nia_assert_is_ok(&remove_modifier_with_values(
                &mut interpreter,
                device_id_value,
                key_code_value,
            ));

            let expected =
                interpreter.execute_in_main_environment(code).unwrap();
            let result =
                library::get_defined_modifiers_as_value(&mut interpreter)
                    .unwrap();

            crate::utils::assert_deep_equal(&mut interpreter, expected, result);
        }
    }

    #[test]
    fn returns_generic_execution_error_when_there_is_no_modifier() {
        let mut interpreter = Interpreter::new();

        let device_id = Some(Value::Integer(3));
        let key_code = Value::Integer(24);

        let result =
            remove_modifier_with_values(&mut interpreter, device_id, key_code);

        crate::utils::assert_generic_execution_error(&result);
    }
}
