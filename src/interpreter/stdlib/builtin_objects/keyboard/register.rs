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
    if values.len() != 3 {
        return Error::invalid_argument_count_error(
            "Built-in function `keyboard:register' takes three arguments exactly.",
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

    #[test]
    fn adds_keyboards() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (r#"'()"#, "nia-defined-keyboards"),
            (
                r#"nil"#,
                r#"(keyboard:register 3 "/dev/input/event1" "Keyboard 1")"#,
            ),
            (
                r#"(list '(3 "/dev/input/event1" "Keyboard 1"))"#,
                "nia-defined-keyboards",
            ),
            (
                r#"nil"#,
                r#"(keyboard:register 2 "/dev/input/event2" "Keyboard 2")"#,
            ),
            (
                r#"(list '(2 "/dev/input/event2" "Keyboard 2") '(3 "/dev/input/event1" "Keyboard 1"))"#,
                "nia-defined-keyboards",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(keyboard:register 1.1 \"path\" \"name\")",
            "(keyboard:register #t \"path\" \"name\")",
            "(keyboard:register #f \"path\" \"name\")",
            "(keyboard:register :keyword \"path\" \"name\")",
            "(keyboard:register 'symbol \"path\" \"name\")",
            "(keyboard:register '(list) \"path\" \"name\")",
            "(keyboard:register {} \"path\" \"name\")",
            "(keyboard:register #() \"path\" \"name\")",
            "(keyboard:register 0 1 \"name\")",
            "(keyboard:register 0 1.1 \"name\")",
            "(keyboard:register 0 #t \"name\")",
            "(keyboard:register 0 #f \"name\")",
            "(keyboard:register 0 :keyword \"name\")",
            "(keyboard:register 0 'symbol \"name\")",
            "(keyboard:register 0 '(1 2) \"name\")",
            "(keyboard:register 0 {} \"name\")",
            "(keyboard:register 0 #() \"name\")",
            "(keyboard:register 0 \"path\" 1)",
            "(keyboard:register 0 \"path\" 1.1)",
            "(keyboard:register 0 \"path\" #t)",
            "(keyboard:register 0 \"path\" #f)",
            "(keyboard:register 0 \"path\" :keyword)",
            "(keyboard:register 0 \"path\" 'symbol)",
            "(keyboard:register 0 \"path\" '(1 2))",
            "(keyboard:register 0 \"path\" {})",
            "(keyboard:register 0 \"path\" #())",
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
            "(keyboard:register 0)",
            "(keyboard:register 0 \"path\")",
            "(keyboard:register 0 \"path\" \"name\" '())",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
