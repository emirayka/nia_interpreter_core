use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn register(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `keyboard:register' takes two or three arguments.",
        )
        .into();
    }

    let mut values = values;

    let path = library::read_as_string(interpreter, values.remove(0))?.clone();

    let name = library::read_as_string(interpreter, values.remove(0))?.clone();

    library::register_keyboard(interpreter, path, name)?;

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn adds_keyboards() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (r#"'()"#, "nia-registered-keyboards"),
            (
                r#"nil"#,
                r#"(keyboard:register "/dev/input/event1" "Keyboard 1")"#,
            ),
            (
                r#"(list '("/dev/input/event1" "Keyboard 1"))"#,
                "nia-registered-keyboards",
            ),
            (
                r#"nil"#,
                r#"(keyboard:register "/dev/input/event2" "Keyboard 2")"#,
            ),
            (
                r#"(list '("/dev/input/event2" "Keyboard 2") '("/dev/input/event1" "Keyboard 1"))"#,
                "nia-registered-keyboards",
            ),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(keyboard:register 1 \"name\")",
            "(keyboard:register 1.1 \"name\")",
            "(keyboard:register #t \"name\")",
            "(keyboard:register #f \"name\")",
            "(keyboard:register :keyword \"name\")",
            "(keyboard:register 'symbol \"name\")",
            "(keyboard:register '(1 2) \"name\")",
            "(keyboard:register {} \"name\")",
            "(keyboard:register #() \"name\")",
            "(keyboard:register \"path\" 1)",
            "(keyboard:register \"path\" 1.1)",
            "(keyboard:register \"path\" #t)",
            "(keyboard:register \"path\" #f)",
            "(keyboard:register \"path\" :keyword)",
            "(keyboard:register \"path\" 'symbol)",
            "(keyboard:register \"path\" '(1 2))",
            "(keyboard:register \"path\" {})",
            "(keyboard:register \"path\" #())",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(keyboard:register)",
            "(keyboard:register \"path\")",
            "(keyboard:register \"path\" \"name\" '())",
        ];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
