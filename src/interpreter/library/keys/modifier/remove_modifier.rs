use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn remove_modifier(
    interpreter: &mut Interpreter,
    device_id: i32,
    key_code: i32,
) -> Result<(), Error> {
    let device_id_value = Value::Integer(device_id as i64);
    let key_code_value = Value::Integer(key_code as i64);

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

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn removes_defined_modifiers() {
        let mut interpreter = Interpreter::new();

        let specs = vec![(3, 1, "1"), (2, 2, ""), (1, 3, "3")];

        for spec in specs {
            nia_assert_is_ok(&library::define_modifier(
                &mut interpreter,
                spec.0,
                spec.1,
                spec.2,
            ));
        }

        let expected = interpreter
            .execute_in_main_environment(r#"'((1 3 "3") (2 2 ()) (3 1 "1"))"#)
            .unwrap();
        let result =
            library::get_defined_modifiers_as_values(&mut interpreter).unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result);

        let specs = vec![
            (2, 2, r#"'((1 3 "3") (3 1 "1"))"#),
            (1, 3, r#"'((3 1 "1"))"#),
            (3, 1, r#"'()"#),
        ];

        for spec in specs {
            nia_assert_is_ok(&remove_modifier(
                &mut interpreter,
                spec.0,
                spec.1,
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
    fn returns_generic_execution_error_when_there_is_no_such_modifier() {
        let mut interpreter = Interpreter::new();

        let device_id = 2;
        let key_code = 23;
        let modifier_alias = "mod";

        nia_assert_is_ok(&library::define_modifier(
            &mut interpreter,
            device_id,
            key_code,
            modifier_alias,
        ));

        let device_id = 2;
        let key_code = 24;
        let result = remove_modifier(&mut interpreter, device_id, key_code);

        crate::utils::assert_generic_execution_error(&result);
    }
}
