use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn get_defined_keyboards(
    interpreter: &mut Interpreter,
) -> Result<Value, Error> {
    let keyboard_list =
        library::get_root_variable(interpreter, "nia-registered-keyboards")
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

        nia_assert_is_ok(&library::define_keyboard_with_strings(
            &mut interpreter,
            "/dev/input/event6",
            "first",
        ));

        let result = library::get_root_variable(
            &mut interpreter,
            "nia-registered-keyboards",
        )
        .unwrap();
        let expected = interpreter
            .execute_in_main_environment(r#"'(("/dev/input/event6" "first"))"#)
            .unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result);

        nia_assert_is_ok(&library::define_keyboard_with_strings(
            &mut interpreter,
            "/dev/input/event66",
            "second",
        ));

        let result = library::get_root_variable(
            &mut interpreter,
            "nia-registered-keyboards",
        )
        .unwrap();
        let expected = interpreter
            .execute_in_main_environment(r#"'(("/dev/input/event66" "second") ("/dev/input/event6" "first"))"#)
            .unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result);
    }
}
