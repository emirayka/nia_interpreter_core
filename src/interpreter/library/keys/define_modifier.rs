use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn define_modifier<S>(
    interpreter: &mut Interpreter,
    device_id: i32,
    key_code: i32,
    modifier_alias: S,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let modifier_alias_str = modifier_alias.as_ref();

    let device_id_value = Value::Integer(device_id as i64);
    let key_code_value = Value::Integer(key_code as i64);
    let modifier_alias_value = if modifier_alias_str.len() == 0 {
        interpreter.intern_nil_symbol_value()
    } else {
        interpreter.intern_string_value(modifier_alias_str)
    };

    library::define_modifier_with_values(
        interpreter,
        device_id_value,
        key_code_value,
        modifier_alias_value,
    )
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
            (3, 1, "", r#"'((3 1 ()))"#),
            (2, 2, "bb", r#"'((2 2 "bb") (3 1 ()))"#),
            (1, 3, "cc", r#"'((1 3 "cc") (2 2 "bb") (3 1 ()))"#),
        ];

        for spec in specs {
            nia_assert_is_ok(&define_modifier(
                &mut interpreter,
                spec.0,
                spec.1,
                spec.2,
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

        let device_id = 1;
        let key_code = 23;
        let modifier_alias = "mod";

        nia_assert_is_ok(&define_modifier(
            &mut interpreter,
            device_id,
            key_code,
            modifier_alias,
        ));

        let result = &define_modifier(
            &mut interpreter,
            device_id,
            key_code,
            modifier_alias,
        );

        crate::utils::assert_generic_execution_error(&result);
    }
}
