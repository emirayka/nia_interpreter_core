use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

fn find_index(
    interpreter: &mut Interpreter,
    modifiers_vector: &Vec<Value>,
    keyboard_path: Value,
    key_code: Value,
) -> Result<usize, Error> {
    let mut index = 0;

    for modifier_list in modifiers_vector {
        let modifier_vector =
            library::read_as_vector(interpreter, *modifier_list)?;

        if modifier_vector.len() != 3 {
            return Error::generic_execution_error(
                "Invariant violation: `nia-defined-modifiers' must be a list of three-element lists."
            ).into();
        }

        if library::deep_equal(interpreter, keyboard_path, modifier_vector[0])?
            && library::deep_equal(interpreter, key_code, modifier_vector[1])?
        {
            break;
        }

        index += 1;
    }

    Ok(index)
}

pub fn remove_modifier_with_values(
    interpreter: &mut Interpreter,
    keyboard_path: Value,
    key_code: Value,
) -> Result<(), Error> {
    library::check_value_is_integer(keyboard_path)?;
    library::check_value_is_integer(key_code)?;

    let root_environment_id = interpreter.get_root_environment_id();
    let symbol_id_defined_modifiers =
        interpreter.intern_symbol_id("nia-defined-modifiers");
    let modifiers_list = library::get_defined_modifiers_as_values(interpreter)?;
    let mut modifiers_vector =
        library::read_as_vector(interpreter, modifiers_list)?;

    let index =
        find_index(interpreter, &modifiers_vector, keyboard_path, key_code)?;

    if index < modifiers_vector.len() {
        modifiers_vector.remove(index);

        let new_list = interpreter.vec_to_list(modifiers_vector);

        interpreter.set_variable(
            root_environment_id,
            symbol_id_defined_modifiers,
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

        let specs = vec![(3, 1), (2, 2), (1, 3)];

        for spec in specs {
            let keyboard_path = Value::Integer(spec.0);
            let key_code = Value::Integer(spec.1);
            let modifier_alias = interpreter.intern_nil_symbol_value();

            nia_assert_is_ok(&library::define_modifier_with_values(
                &mut interpreter,
                keyboard_path,
                key_code,
                modifier_alias,
            ));
        }

        let expected = interpreter
            .execute_in_main_environment(r#"'((1 3 ()) (2 2 ()) (3 1 ()))"#)
            .unwrap();
        let result =
            library::get_defined_modifiers_as_values(&mut interpreter).unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result);

        let specs = vec![
            (2, 2, r#"'((1 3 ()) (3 1 ()))"#),
            (1, 3, r#"'((3 1 ()))"#),
            (3, 1, r#"'()"#),
        ];

        for spec in specs {
            let keyboard_path = Value::Integer(spec.0);
            let key_code = Value::Integer(spec.1);

            nia_assert_is_ok(&remove_modifier_with_values(
                &mut interpreter,
                keyboard_path,
                key_code,
            ));

            let expected =
                interpreter.execute_in_main_environment(spec.2).unwrap();
            let result =
                library::get_defined_modifiers_as_values(&mut interpreter)
                    .unwrap();

            crate::utils::assert_deep_equal(&mut interpreter, expected, result);
        }
    }

    #[test]
    fn returns_generic_execution_error_when_there_is_no_modifier() {
        let mut interpreter = Interpreter::new();

        let device_id = Value::Integer(3);
        let key_code = Value::Integer(23);
        let modifier_alias = interpreter.intern_string_value("mod");

        nia_assert_is_ok(&library::define_modifier_with_values(
            &mut interpreter,
            device_id,
            key_code,
            modifier_alias,
        ));

        let device_id = Value::Integer(3);
        let key_code = Value::Integer(24);
        let modifier_alias = interpreter.intern_string_value("mod");

        let result =
            remove_modifier_with_values(&mut interpreter, device_id, key_code);

        crate::utils::assert_generic_execution_error(&result);
    }
}
