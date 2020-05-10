use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn register_keyboard<S>(
    interpreter: &mut Interpreter,
    keyboard_path: S,
    keyboard_name: S,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let root_environment_id = interpreter.get_main_environment_id();
    let symbol_id_registered_keyboards =
        interpreter.intern_symbol_id("nia-registered-keyboards");

    let keyboard_list = interpreter
        .lookup_variable(root_environment_id, symbol_id_registered_keyboards)?
        .ok_or_else(|| {
            Error::generic_execution_error("Cannot find registered_keyboards")
        })?;

    let keyboard_path_value = keyboard_path.as_ref();
    let keyboard_name_value = keyboard_name.as_ref();

    let keyboard_path_value =
        interpreter.intern_string_value(keyboard_path_value);
    let keyboard_name_value =
        interpreter.intern_string_value(keyboard_name_value);

    let new_list =
        interpreter.vec_to_list(vec![keyboard_path_value, keyboard_name_value]);
    let cons = interpreter.make_cons_value(new_list, keyboard_list);

    interpreter.set_variable(
        root_environment_id,
        symbol_id_registered_keyboards,
        cons,
    )?;

    Ok(())
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

        nia_assert_is_ok(&register_keyboard(
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

        crate::utils::assert_deep_equal(&mut interpreter, expected, result)
    }

    #[test]
    fn returns_generic_error_when_keyboard_with_the_same_path_was_already_defined(
    ) {
        let mut interpreter = Interpreter::new();

        nia_assert_is_ok(&register_keyboard(
            &mut interpreter,
            "/dev/input/event6",
            "first",
        ));

        let result =
            register_keyboard(&mut interpreter, "/dev/input/event6", "second");
    }
}
