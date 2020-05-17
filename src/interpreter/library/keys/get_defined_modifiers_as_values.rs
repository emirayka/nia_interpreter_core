use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn get_defined_modifiers_as_values(
    interpreter: &mut Interpreter,
) -> Result<Value, Error> {
    let keyboard_list =
        library::get_root_variable(interpreter, "nia-defined-modifiers")
            .map_err(|err| {
                Error::generic_execution_error_caused(
                    "Cannot read registered keyboards.",
                    err,
                )
            })?;

    Ok(keyboard_list)
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

        let result = get_defined_modifiers_as_values(&mut interpreter).unwrap();
        let expected =
            interpreter.execute_in_main_environment(r#"'()"#).unwrap();
        crate::utils::assert_deep_equal(&mut interpreter, expected, result);

        let keyboard_name = Value::Integer(3);
        let key_code = Value::Integer(22);
        let modifier_alias = interpreter.intern_nil_symbol_value();
        nia_assert_is_ok(&library::define_modifier_with_values(
            &mut interpreter,
            keyboard_name,
            key_code,
            modifier_alias,
        ));

        let result = get_defined_modifiers_as_values(&mut interpreter).unwrap();
        let expected = interpreter
            .execute_in_main_environment(r#"'((3 22 ()))"#)
            .unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result);

        let keyboard_name = Value::Integer(2);
        let key_code = Value::Integer(33);
        let modifier_alias = interpreter.intern_string_value("mod");
        nia_assert_is_ok(&library::define_modifier_with_values(
            &mut interpreter,
            keyboard_name,
            key_code,
            modifier_alias,
        ));

        let result = get_defined_modifiers_as_values(&mut interpreter).unwrap();
        let expected = interpreter
            .execute_in_main_environment(r#"'((2 33 "mod") (3 22 ()))"#)
            .unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result);
    }
}
