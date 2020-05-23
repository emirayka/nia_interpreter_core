use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn define(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 3 {
        return Error::invalid_argument_count_error(
            "Built-in function `device:define' takes three arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    library::define_keyboard_with_values(
        interpreter,
        values.remove(0),
        values.remove(0),
        values.remove(0),
    )?;

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;
    use crate::DEFINED_DEVICES_ROOT_VARIABLE_NAME;

    #[test]
    fn adds_keyboards() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (r#"'()"#, DEFINED_DEVICES_ROOT_VARIABLE_NAME),
            (
                r#"nil"#,
                r#"(device:define 3 "/dev/input/event1" "Keyboard 1")"#,
            ),
            (
                r#"(list:new '(3 "/dev/input/event1" "Keyboard 1"))"#,
                DEFINED_DEVICES_ROOT_VARIABLE_NAME,
            ),
            (
                r#"nil"#,
                r#"(device:define 2 "/dev/input/event2" "Keyboard 2")"#,
            ),
            (
                r#"(list:new '(2 "/dev/input/event2" "Keyboard 2") '(3 "/dev/input/event1" "Keyboard 1"))"#,
                DEFINED_DEVICES_ROOT_VARIABLE_NAME,
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(device:define 1.1 \"path\" \"name\")",
            "(device:define #t \"path\" \"name\")",
            "(device:define #f \"path\" \"name\")",
            "(device:define :keyword \"path\" \"name\")",
            "(device:define 'symbol \"path\" \"name\")",
            "(device:define '(list:new) \"path\" \"name\")",
            "(device:define {} \"path\" \"name\")",
            "(device:define #() \"path\" \"name\")",
            "(device:define 0 1 \"name\")",
            "(device:define 0 1.1 \"name\")",
            "(device:define 0 #t \"name\")",
            "(device:define 0 #f \"name\")",
            "(device:define 0 :keyword \"name\")",
            "(device:define 0 'symbol \"name\")",
            "(device:define 0 '(1 2) \"name\")",
            "(device:define 0 {} \"name\")",
            "(device:define 0 #() \"name\")",
            "(device:define 0 \"path\" 1)",
            "(device:define 0 \"path\" 1.1)",
            "(device:define 0 \"path\" #t)",
            "(device:define 0 \"path\" #f)",
            "(device:define 0 \"path\" :keyword)",
            "(device:define 0 \"path\" 'symbol)",
            "(device:define 0 \"path\" '(1 2))",
            "(device:define 0 \"path\" {})",
            "(device:define 0 \"path\" #())",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(device:define 0)",
            "(device:define 0 \"path\")",
            "(device:define 0 \"path\" \"name\" '())",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
