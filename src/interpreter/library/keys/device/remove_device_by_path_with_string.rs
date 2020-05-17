use crate::Error;
use crate::Interpreter;

use crate::library;

pub fn remove_keyboard_by_path_with_string<S>(
    interpreter: &mut Interpreter,
    keyboard_path: S,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let keyboard_path_string = keyboard_path.as_ref();
    println!("Removing keyboard by path: {}", keyboard_path_string);

    let keyboard_path_value =
        interpreter.intern_string_value(keyboard_path_string);

    library::remove_keyboard_by_path_with_value(
        interpreter,
        keyboard_path_value,
    )
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    fn define_keyboards(
        interpreter: &mut Interpreter,
        keyboards: Vec<(i32, &str, &str)>,
    ) {
        for (keyboard_id, keyboard_path, keyboard_name) in keyboards {
            nia_assert_is_ok(&library::define_device(
                interpreter,
                keyboard_id,
                keyboard_path,
                keyboard_name,
            ))
        }
    }

    fn assert_defined_keyboards_equal(
        interpreter: &mut Interpreter,
        spec_code: &str,
    ) {
        let result = library::get_defined_keyboards(interpreter).unwrap();
        let expected =
            interpreter.execute_in_main_environment(spec_code).unwrap();

        crate::utils::assert_deep_equal(interpreter, expected, result);
    }

    #[test]
    fn removes_keyboard_from_registered_keyboards() {
        let mut interpreter = Interpreter::new();

        let keyboards = vec![
            (1, "/dev/input/event3", "third"),
            (2, "/dev/input/event2", "second"),
            (3, "/dev/input/event1", "first"),
        ];

        let specs = vec![
            (
                r#"/dev/input/event1"#,
                r#"'((2 "/dev/input/event2" "second") (1 "/dev/input/event3" "third"))"#,
            ),
            (
                r#"/dev/input/event3"#,
                r#"'((2 "/dev/input/event2" "second"))"#,
            ),
            (r#"/dev/input/event2"#, r#"'()"#),
        ];

        define_keyboards(&mut interpreter, keyboards);
        assert_defined_keyboards_equal(
            &mut interpreter,
            r#"'((3 "/dev/input/event1" "first") (2 "/dev/input/event2" "second") (1 "/dev/input/event3" "third"))"#,
        );

        for (path_for_deletion, expected) in specs {
            nia_assert_is_ok(&remove_keyboard_by_path_with_string(
                &mut interpreter,
                path_for_deletion,
            ));

            assert_defined_keyboards_equal(&mut interpreter, expected);
        }
    }

    #[test]
    fn returns_generic_error_when_there_are_no_keyboard_with_path() {
        let mut interpreter = Interpreter::new();

        let keyboards = vec![
            (1, "/dev/input/event3", "third"),
            (2, "/dev/input/event2", "second"),
            (3, "/dev/input/event1", "first"),
        ];

        define_keyboards(&mut interpreter, keyboards);
        assert_defined_keyboards_equal(
            &mut interpreter,
            r#"'((3 "/dev/input/event1" "first") (2 "/dev/input/event2" "second") (1 "/dev/input/event3" "third"))"#,
        );

        let result = remove_keyboard_by_path_with_string(
            &mut interpreter,
            "/dev/non-input/arst",
        );

        crate::utils::assert_generic_execution_error(&result);
    }
}
