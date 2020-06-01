use crate::{Error, Interpreter};
use crate::{Value, DEFINED_DEVICES_ROOT_VARIABLE_NAME};

use crate::library;

fn find_target_keyboard_index(
    interpreter: &mut Interpreter,
    keyboard_lists_vector: &Vec<Value>,
    keyboard_id_value: Value,
) -> Result<usize, Error> {
    let iter = keyboard_lists_vector.iter();
    let mut index = 0;

    for keyboard_list in iter {
        let keyboard_vector =
            library::read_as_vector(interpreter, *keyboard_list)?;

        if keyboard_vector.len() != 3 {
            return Error::generic_execution_error(
                format!("Invariant violated: `nia-defined-keyboards' must be a list of three elements list.")
            ).into();
        }

        if library::deep_equal(
            interpreter,
            keyboard_vector[0],
            keyboard_id_value,
        )? {
            break;
        }

        index += 1;
    }

    Ok(index)
}

pub fn remove_keyboard_by_id_with_value(
    interpreter: &mut Interpreter,
    keyboard_id_value: Value,
) -> Result<(), Error> {
    library::check_value_is_integer(keyboard_id_value)?;

    let root_environment_id = interpreter.get_root_environment_id();
    let symbol_id_registered_keyboards =
        interpreter.intern_symbol_id(DEFINED_DEVICES_ROOT_VARIABLE_NAME);

    let keyboard_list = interpreter
        .lookup_variable(root_environment_id, symbol_id_registered_keyboards)?
        .ok_or_else(|| {
            Error::generic_execution_error("Cannot find registered_keyboards")
        })?;

    let mut keyboard_lists_vector =
        library::read_as_vector(interpreter, keyboard_list)?;

    let index = find_target_keyboard_index(
        interpreter,
        &keyboard_lists_vector,
        keyboard_id_value,
    )?;

    if index < keyboard_lists_vector.len() {
        keyboard_lists_vector.remove(index);
        let result = interpreter.vec_to_list(keyboard_lists_vector);

        interpreter.set_variable(
            root_environment_id,
            symbol_id_registered_keyboards,
            result,
        )?;

        Ok(())
    } else {
        Error::generic_execution_error(
            "Cannot find registered keyboard with provided identifier.",
        )
        .into()
    }
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
        let result = library::get_defined_devices_list(interpreter).unwrap();
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
                Value::Integer(3),
                r#"'((2 "/dev/input/event2" "second") (1 "/dev/input/event3" "third"))"#,
            ),
            (Value::Integer(1), r#"'((2 "/dev/input/event2" "second"))"#),
            (Value::Integer(2), r#"'()"#),
        ];

        define_keyboards(&mut interpreter, keyboards);
        assert_defined_keyboards_equal(
            &mut interpreter,
            r#"'((3 "/dev/input/event1" "first") (2 "/dev/input/event2" "second") (1 "/dev/input/event3" "third"))"#,
        );

        for (id_for_deletion, expected) in specs {
            nia_assert_is_ok(&remove_keyboard_by_id_with_value(
                &mut interpreter,
                id_for_deletion,
            ));

            assert_defined_keyboards_equal(&mut interpreter, expected);
        }
    }

    #[test]
    fn returns_generic_error_when_there_are_no_keyboard_with_id() {
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

        let keyboard_id_value = Value::Integer(4);

        let result = remove_keyboard_by_id_with_value(
            &mut interpreter,
            keyboard_id_value,
        );

        crate::utils::assert_generic_execution_error(&result);
    }
}
