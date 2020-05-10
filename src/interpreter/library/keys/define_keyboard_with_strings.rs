use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn define_keyboard_with_strings<S>(
    interpreter: &mut Interpreter,
    keyboard_path: S,
    keyboard_name: S,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let keyboard_path_value =
        interpreter.intern_string_value(keyboard_path.as_ref());
    let keyboard_name_value =
        interpreter.intern_string_value(keyboard_name.as_ref());

    library::define_keyboard_with_values(
        interpreter,
        keyboard_path_value,
        keyboard_name_value,
    )
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn changes_variable_registered_keyboards() {
        let mut interpreter = Interpreter::new();

        nia_assert_is_ok(&define_keyboard_with_strings(
            &mut interpreter,
            "/dev/input/event6",
            "first",
        ));

        let result = library::get_root_variable(
            &mut interpreter,
            "nia-defined-keyboards",
        )
        .unwrap();
        let expected = interpreter
            .execute_in_main_environment(r#"'(("/dev/input/event6" "first"))"#)
            .unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result)
    }

    #[test]
    fn returns_generic_error_when_keyboard_with_the_same_path_was_already_defined(
    ) {
        let mut interpreter = Interpreter::new();

        nia_assert_is_ok(&define_keyboard_with_strings(
            &mut interpreter,
            "/dev/input/event6",
            "first",
        ));

        let result = define_keyboard_with_strings(
            &mut interpreter,
            "/dev/input/event6",
            "second",
        );

        crate::utils::assert_generic_execution_error(&result);
    }

    #[test]
    fn returns_generic_error_when_keyboard_with_the_same_name_was_already_defined(
    ) {
        let mut interpreter = Interpreter::new();

        nia_assert_is_ok(&define_keyboard_with_strings(
            &mut interpreter,
            "/dev/input/event6",
            "first",
        ));

        let result = define_keyboard_with_strings(
            &mut interpreter,
            "/dev/input/event7",
            "first",
        );

        crate::utils::assert_generic_execution_error(&result);
    }
}
