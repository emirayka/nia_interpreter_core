use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn define_device<S>(
    interpreter: &mut Interpreter,
    device_id: i32,
    device_path: S,
    device_name: S,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let device_id_value = Value::Integer(device_id as i64);
    let device_path_value =
        interpreter.intern_string_value(device_path.as_ref());
    let device_name_value =
        interpreter.intern_string_value(device_name.as_ref());

    library::define_keyboard_with_values(
        interpreter,
        device_id_value,
        device_path_value,
        device_name_value,
    )
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    use crate::DEFINED_DEVICES_ROOT_VARIABLE_NAME;
    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn changes_variable_registered_keyboards() {
        let mut interpreter = Interpreter::new();

        nia_assert_is_ok(&define_device(
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

        crate::utils::assert_deep_equal(&mut interpreter, expected, result)
    }

    #[test]
    fn returns_generic_error_when_keyboard_with_the_same_id_was_already_defined(
    ) {
        let mut interpreter = Interpreter::new();

        nia_assert_is_ok(&define_device(
            &mut interpreter,
            0,
            "/dev/input/event6",
            "first",
        ));

        let result =
            define_device(&mut interpreter, 0, "/dev/input/event7", "second");

        crate::utils::assert_generic_execution_error(&result);
    }

    #[test]
    fn returns_generic_error_when_keyboard_with_the_same_path_was_already_defined(
    ) {
        let mut interpreter = Interpreter::new();

        nia_assert_is_ok(&define_device(
            &mut interpreter,
            0,
            "/dev/input/event6",
            "first",
        ));

        let result =
            define_device(&mut interpreter, 1, "/dev/input/event6", "second");

        crate::utils::assert_generic_execution_error(&result);
    }

    #[test]
    fn returns_generic_error_when_keyboard_with_the_same_name_was_already_defined(
    ) {
        let mut interpreter = Interpreter::new();

        nia_assert_is_ok(&define_device(
            &mut interpreter,
            0,
            "/dev/input/event6",
            "first",
        ));

        let result =
            define_device(&mut interpreter, 1, "/dev/input/event7", "first");

        crate::utils::assert_generic_execution_error(&result);
    }
}
