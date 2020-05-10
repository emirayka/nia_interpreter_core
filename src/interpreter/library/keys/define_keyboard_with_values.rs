use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

fn check_keyboard_can_be_registered(
    interpreter: &mut Interpreter,
    keyboard_list_value: Value,
    keyboard_path_value: Value,
    keyboard_name_value: Value,
) -> Result<(), Error> {
    let registered_keyboards =
        library::read_as_vector(interpreter, keyboard_list_value)?;

    for registered_keyboard in registered_keyboards {
        let vec = library::read_as_vector(interpreter, registered_keyboard)?;

        if vec.len() != 2 {
            return Error::generic_execution_error(
                format!("Invariant is violated: `nia-defined-keyboards' must be a list of two-element lists.")
            ).into();
        }

        if vec[0] == keyboard_path_value {
            return Error::generic_execution_error(format!(
                "Keyboard with path {} was already defined.",
                keyboard_path_value
            ))
            .into();
        }

        if vec[1] == keyboard_name_value {
            return Error::generic_execution_error(format!(
                "Keyboard with path {} was already defined.",
                keyboard_path_value
            ))
            .into();
        }
    }

    Ok(())
}

pub fn define_keyboard_with_values(
    interpreter: &mut Interpreter,
    keyboard_path_value: Value,
    keyboard_name_value: Value,
) -> Result<(), Error> {
    library::check_value_is_string(keyboard_path_value)?;
    library::check_value_is_string(keyboard_name_value)?;

    let root_environment_id = interpreter.get_root_environment_id();
    let symbol_id_registered_keyboards =
        interpreter.intern_symbol_id("nia-defined-keyboards");

    let keyboard_list = interpreter
        .lookup_variable(root_environment_id, symbol_id_registered_keyboards)?
        .ok_or_else(|| {
            Error::generic_execution_error("Cannot find registered_keyboards")
        })?;

    check_keyboard_can_be_registered(
        interpreter,
        keyboard_list,
        keyboard_path_value,
        keyboard_name_value,
    )?;

    let new_list =
        interpreter.vec_to_list(vec![keyboard_path_value, keyboard_name_value]);
    let cons = interpreter.make_cons_value(new_list, keyboard_list);

    interpreter.set_variable(
        root_environment_id,
        symbol_id_registered_keyboards,
        cons,
    )?;

    library::print_value(interpreter, cons);

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

        let keyboard_path_value =
            interpreter.intern_string_value("/dev/input/event6");
        let keyboard_name_value = interpreter.intern_string_value("first");

        nia_assert_is_ok(&define_keyboard_with_values(
            &mut interpreter,
            keyboard_path_value,
            keyboard_name_value,
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

        let keyboard_path_value =
            interpreter.intern_string_value("/dev/input/event6");
        let keyboard_name_value = interpreter.intern_string_value("first");

        nia_assert_is_ok(&define_keyboard_with_values(
            &mut interpreter,
            keyboard_path_value,
            keyboard_name_value,
        ));

        let keyboard_path_value =
            interpreter.intern_string_value("/dev/input/event6");
        let keyboard_name_value = interpreter.intern_string_value("second");

        let result = define_keyboard_with_values(
            &mut interpreter,
            keyboard_path_value,
            keyboard_name_value,
        );

        crate::utils::assert_generic_execution_error(&result);
    }

    #[test]
    fn returns_generic_error_when_keyboard_with_the_same_name_was_already_defined(
    ) {
        let mut interpreter = Interpreter::new();

        let keyboard_path_value =
            interpreter.intern_string_value("/dev/input/event6");
        let keyboard_name_value = interpreter.intern_string_value("first");

        nia_assert_is_ok(&define_keyboard_with_values(
            &mut interpreter,
            keyboard_path_value,
            keyboard_name_value,
        ));

        let keyboard_path_value =
            interpreter.intern_string_value("/dev/input/event7");
        let keyboard_name_value = interpreter.intern_string_value("first");

        let result = define_keyboard_with_values(
            &mut interpreter,
            keyboard_path_value,
            keyboard_name_value,
        );

        crate::utils::assert_generic_execution_error(&result);
    }
}
