use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn remove_modifier<S>(
    interpreter: &mut Interpreter,
    keyboard_path: S,
    key_code: i32,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let keyboard_path_string = keyboard_path.as_ref();
    let keyboard_path_value =
        interpreter.intern_string_value(keyboard_path_string);
    let key_code_value = Value::Integer(key_code as i64);

    library::remove_modifier_with_values(
        interpreter,
        keyboard_path_value,
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

        let specs = vec![
            ("keyboard1", 1, "1"),
            ("keyboard2", 2, ""),
            ("keyboard3", 3, "3"),
        ];

        for spec in specs {
            nia_assert_is_ok(&library::define_modifier(
                &mut interpreter,
                spec.0,
                spec.1,
                spec.2,
            ));
        }

        let expected = interpreter
            .execute_in_main_environment(
                r#"'(("keyboard3" 3 "3") ("keyboard2" 2 ()) ("keyboard1" 1 "1"))"#,
            )
            .unwrap();
        let result =
            library::get_defined_modifiers_as_values(&mut interpreter).unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result);

        let specs = vec![
            (
                "keyboard2",
                2,
                r#"'(("keyboard3" 3 "3") ("keyboard1" 1 "1"))"#,
            ),
            ("keyboard3", 3, r#"'(("keyboard1" 1 "1"))"#),
            ("keyboard1", 1, r#"'()"#),
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

        let keyboard_path = "keyboard2";
        let key_code = 23;
        let modifier_alias = "mod";

        nia_assert_is_ok(&library::define_modifier(
            &mut interpreter,
            keyboard_path,
            key_code,
            modifier_alias,
        ));

        let keyboard_path = "keyboard2";
        let key_code = 24;
        let modifier_alias = "mod";
        let result = remove_modifier(&mut interpreter, keyboard_path, key_code);

        crate::utils::assert_generic_execution_error(&result);
    }
}
