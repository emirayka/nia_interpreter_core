use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

fn check_modifier_can_be_defined(
    interpreter: &mut Interpreter,
    keyboard_path: Value,
    key_code: Value,
) -> Result<(), Error> {
    let modifiers_list = library::get_defined_modifiers_as_values(interpreter)?;

    let modifiers_vector =
        library::read_as_vector(interpreter, modifiers_list)?;

    for modifier_list in modifiers_vector {
        let modifier_vector =
            library::read_as_vector(interpreter, modifier_list)?;

        if modifier_vector.len() != 3 {
            return Error::generic_execution_error(
                "Invariant violation: `nia-defined-modifiers' must be a list of three-element lists."
            ).into();
        }

        if !library::deep_equal(interpreter, keyboard_path, modifier_vector[0])?
        {
            continue;
        }

        if !library::deep_equal(interpreter, key_code, modifier_vector[1])? {
            continue;
        }

        return Error::generic_execution_error("Modifier was already defined.")
            .into();
    }

    Ok(())
}

pub fn define_modifier_with_values(
    interpreter: &mut Interpreter,
    keyboard_path: Value,
    key_code: Value,
    modifier_alias: Value,
) -> Result<(), Error> {
    library::check_value_is_string(keyboard_path)?;
    library::check_value_is_integer(key_code)?;
    library::check_value_is_string_or_nil(interpreter, modifier_alias)?;

    check_modifier_can_be_defined(interpreter, keyboard_path, key_code)?;

    let new_modifier_list =
        interpreter.vec_to_list(vec![keyboard_path, key_code, modifier_alias]);

    library::add_value_to_root_list(
        interpreter,
        "nia-defined-modifiers",
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
            library::get_defined_modifiers_as_values(&mut interpreter).unwrap();
        let expected =
            interpreter.execute_in_main_environment(r#"'()"#).unwrap();

        let specs = vec![
            ("a", 1, "", r#"'(("a" 1 ()))"#),
            ("b", 2, "bb", r#"'(("b" 2 "bb") ("a" 1 ()))"#),
            ("c", 3, "cc", r#"'(("c" 3 "cc") ("b" 2 "bb") ("a" 1 ()))"#),
        ];

        for spec in specs {
            let keyboard_path = interpreter.intern_string_value(spec.0);
            let key_code = Value::Integer(spec.1);
            let modifier_alias = if spec.2.len() == 0 {
                interpreter.intern_nil_symbol_value()
            } else {
                interpreter.intern_string_value(spec.2)
            };

            nia_assert_is_ok(&define_modifier_with_values(
                &mut interpreter,
                keyboard_path,
                key_code,
                modifier_alias,
            ));

            let expected =
                interpreter.execute_in_main_environment(spec.3).unwrap();
            let result =
                library::get_defined_modifiers_as_values(&mut interpreter)
                    .unwrap();

            crate::utils::assert_deep_equal(&mut interpreter, expected, result)
        }
    }

    #[test]
    fn returns_generic_execution_error_when_attempts_to_define_already_defined_modifier(
    ) {
        let mut interpreter = Interpreter::new();

        let keyboard_path = interpreter.intern_string_value("keyboard2");
        let key_code = Value::Integer(23);
        let modifier_alias = interpreter.intern_string_value("mod");

        nia_assert_is_ok(&define_modifier_with_values(
            &mut interpreter,
            keyboard_path,
            key_code,
            modifier_alias,
        ));

        let result = &define_modifier_with_values(
            &mut interpreter,
            keyboard_path,
            key_code,
            modifier_alias,
        );

        crate::utils::assert_generic_execution_error(&result);
    }
}
