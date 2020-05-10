use crate::Value;
use crate::{Error, Interpreter};

use crate::library;

pub fn remove_keyboard_by_name_with_string<S>(
    interpreter: &mut Interpreter,
    keyboard_name: S,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let keyboard_name_string = keyboard_name.as_ref();
    let keyboard_name_value =
        interpreter.intern_string_value(keyboard_name_string);

    library::remove_keyboard_by_name_with_value(
        interpreter,
        keyboard_name_value,
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
        keyboards: Vec<(&str, &str)>,
    ) {
        for (keyboard_path, keyboard_name) in keyboards {
            nia_assert_is_ok(&library::define_keyboard_with_strings(
                interpreter,
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

        let mut keyboards = vec![
            ("/dev/input/event3", "third"),
            ("/dev/input/event2", "second"),
            ("/dev/input/event1", "first"),
        ];

        let mut specs = vec![
            (
                "first",
                r#"'(("/dev/input/event2" "second") ("/dev/input/event3" "third"))"#,
            ),
            ("third", r#"'(("/dev/input/event2" "second"))"#),
            ("second", r#"'()"#),
        ];

        define_keyboards(&mut interpreter, keyboards);
        assert_defined_keyboards_equal(
            &mut interpreter,
            r#"'(("/dev/input/event1" "first") ("/dev/input/event2" "second") ("/dev/input/event3" "third"))"#,
        );

        for (name_for_deletion, expected) in specs {
            nia_assert_is_ok(&remove_keyboard_by_name_with_string(
                &mut interpreter,
                name_for_deletion,
            ));
        }
    }

    #[test]
    fn returns_generic_error_when_there_are_no_keyboard_with_name() {
        let mut interpreter = Interpreter::new();

        let mut keyboards = vec![
            ("/dev/input/event3", "third"),
            ("/dev/input/event2", "second"),
            ("/dev/input/event1", "first"),
        ];

        define_keyboards(&mut interpreter, keyboards);
        assert_defined_keyboards_equal(
            &mut interpreter,
            r#"'(("/dev/input/event1" "first") ("/dev/input/event2" "second") ("/dev/input/event3" "third"))"#,
        );

        let result =
            remove_keyboard_by_name_with_string(&mut interpreter, "fourth");

        crate::utils::assert_generic_execution_error(&result);
    }
}
