use crate::Interpreter;
use crate::Value;
use crate::{Error, DEFINED_DEVICES_ROOT_VARIABLE_NAME};

use crate::library;

pub fn get_defined_keyboards(
    interpreter: &mut Interpreter,
) -> Result<Value, Error> {
    let keyboard_list = library::get_root_variable(
        interpreter,
        DEFINED_DEVICES_ROOT_VARIABLE_NAME,
    )
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

        nia_assert_is_ok(&library::define_device(
            &mut interpreter,
            0,
            "/dev/input/event6",
            "first",
        ));

        let result = library::get_root_variable(
            &mut interpreter,
            DEFINED_DEVICES_ROOT_VARIABLE_NAME,
        )
        .unwrap();
        let expected = interpreter
            .execute_in_main_environment(
                r#"'((0 "/dev/input/event6" "first"))"#,
            )
            .unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result);

        nia_assert_is_ok(&library::define_device(
            &mut interpreter,
            666,
            "/dev/input/event66",
            "second",
        ));

        let result = library::get_root_variable(
            &mut interpreter,
            DEFINED_DEVICES_ROOT_VARIABLE_NAME,
        )
        .unwrap();
        let expected = interpreter
            .execute_in_main_environment(r#"'((666 "/dev/input/event66" "second") (0 "/dev/input/event6" "first"))"#)
            .unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result);
    }
}
