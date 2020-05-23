use crate::Interpreter;
use crate::Value;
use crate::{Error, Key};

use crate::library;

pub fn remove_modifier(
    interpreter: &mut Interpreter,
    modifier_key: Key,
) -> Result<(), Error> {
    let device_id_value = modifier_key
        .get_device_id()
        .map(|device_id| Value::Integer(device_id as i64));
    let key_code_value = Value::Integer(modifier_key.get_key_id() as i64);

    library::remove_modifier_with_values(
        interpreter,
        device_id_value,
        key_code_value,
    )
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    use crate::ModifierDescription;
    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn removes_defined_modifiers() {
        let mut interpreter = Interpreter::new();

        let keys = vec![nia_key!(3, 1), nia_key!(2), nia_key!(1, 3)];

        let modifiers = keys.iter().map(|key| {
            ModifierDescription::new(*key, format!("{}", key.get_key_id()))
        });

        for modifier in modifiers {
            nia_assert_is_ok(&library::define_modifier(
                &mut interpreter,
                &modifier,
            ));
        }

        let expected = interpreter
            .execute_in_main_environment(r#"'((1 3 "3") (2 "2") (3 1 "1"))"#)
            .unwrap();
        let result =
            library::get_defined_modifiers_as_value(&mut interpreter).unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result);

        let specs = vec![
            (nia_key!(2), r#"'((1 3 "3") (3 1 "1"))"#),
            (nia_key!(1, 3), r#"'((3 1 "1"))"#),
            (nia_key!(3, 1), r#"'()"#),
        ];

        for (key, code) in specs {
            nia_assert_is_ok(&remove_modifier(&mut interpreter, key));

            let expected =
                interpreter.execute_in_main_environment(code).unwrap();
            let result =
                library::get_defined_modifiers_as_value(&mut interpreter)
                    .unwrap();

            crate::utils::assert_deep_equal(&mut interpreter, expected, result);
        }
    }

    #[test]
    fn returns_generic_execution_error_when_there_is_no_such_modifier() {
        let mut interpreter = Interpreter::new();

        let modifier_key = nia_key!(2, 2);

        let result = remove_modifier(&mut interpreter, modifier_key);

        crate::utils::assert_generic_execution_error(&result);
    }
}
