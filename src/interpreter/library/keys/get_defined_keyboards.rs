use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn get_registered_keyboards(
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
    fn changes_variable_registered_keyboards() {
        let mut interpreter = Interpreter::new();

        nia_assert_is_ok(&library::register_keyboard(
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

        crate::utils::assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result,
        )
    }
}
