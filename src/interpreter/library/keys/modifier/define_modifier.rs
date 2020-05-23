use crate::Error;
use crate::Interpreter;
use crate::ModifierDescription;
use crate::Value;

use crate::library;

pub fn define_modifier(
    interpreter: &mut Interpreter,
    modifier: &ModifierDescription,
) -> Result<(), Error> {
    let modifier_alias_str = modifier.get_alias();

    let key = modifier.get_key();

    let device_id_value = modifier
        .get_key()
        .get_device_id()
        .map(|device_id| Value::Integer(device_id as i64));
    let key_code_value = Value::Integer(modifier.get_key().get_key_id() as i64);

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

        library::get_defined_modifiers_as_value(&mut interpreter).unwrap();
        interpreter.execute_in_main_environment(r#"'()"#).unwrap();

        let specs = vec![
            (nia_modifier!(3, 1, ""), r#"'((3 1 ()))"#),
            (nia_modifier!(2, "bb"), r#"'((2 "bb") (3 1 ()))"#),
            (
                nia_modifier!(1, 3, "cc"),
                r#"'((1 3 "cc") (2 "bb") (3 1 ()))"#,
            ),
        ];

        for (modifier, code) in specs {
            nia_assert_is_ok(&define_modifier(&mut interpreter, &modifier));

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
        let modifier = nia_modifier!(1, 3, "kek");

        nia_assert_is_ok(&define_modifier(&mut interpreter, &modifier));

        let result = &define_modifier(&mut interpreter, &modifier);
        crate::utils::assert_generic_execution_error(&result);
    }
}
