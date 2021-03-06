use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn shift_left(
    _interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `bit:shift-left' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let shift = library::read_as_i64(values.remove(0))?;

    if shift < 0 {
        return Error::invalid_argument_error(
            "Built-in function `bit:shift-right' takes positive shift.",
        )
        .into();
    }

    let value = library::read_as_i64(values.remove(0))?;

    let result = match value.checked_shl(shift as u32) {
        Some(result) => result,
        _ => return Error::overflow_error("").into(),
    };

    Ok(Value::Integer(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_shift_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(bit:shift-left 0 0)", "0"),
            ("(bit:shift-left 0 1)", "1"),
            ("(bit:shift-left 0 2)", "2"),
            ("(bit:shift-left 1 0)", "0"),
            ("(bit:shift-left 1 1)", "2"),
            ("(bit:shift-left 1 2)", "4"),
            ("(bit:shift-left 2 0)", "0"),
            ("(bit:shift-left 2 1)", "4"),
            ("(bit:shift-left 2 2)", "8"),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(bit:shift-left 1 1.1)",
            "(bit:shift-left 1 #t)",
            "(bit:shift-left 1 #f)",
            "(bit:shift-left 1 'symbol)",
            "(bit:shift-left 1 \"string\")",
            "(bit:shift-left 1 :keyword)",
            "(bit:shift-left 1 '(s-expression))",
            "(bit:shift-left 1 {})",
            "(bit:shift-left 1 #())",
            "(bit:shift-left 1.1 1)",
            "(bit:shift-left #t 1)",
            "(bit:shift-left #f 1)",
            "(bit:shift-left 'symbol 1)",
            "(bit:shift-left \"string\" 1)",
            "(bit:shift-left :keyword 1)",
            "(bit:shift-left '(s-expression) 1)",
            "(bit:shift-left {} 1)",
            "(bit:shift-left #() 1)",
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
            "(bit:shift-left)",
            "(bit:shift-left 1)",
            "(bit:shift-left 1 2 3)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
