use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn not(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `bit:not' takes two arguments exactly."
        ).into_result();
    }

    let mut values = values;

    let v = library::read_as_i64(
        interpreter,
        values.remove(0)
    )?;

    let result = !v;

    Ok(Value::Integer(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn negates_bitwise() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(bit:not 0)", "-1"),
            ("(bit:not 1)", "-2"),
            ("(bit:not 2)", "-3"),
            ("(bit:not 3)", "-4"),

            ("(bit:not -1)", "0"),
            ("(bit:not -2)", "1"),
            ("(bit:not -3)", "2"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(bit:not 1.1)",
            "(bit:not #t)",
            "(bit:not #f)",
            "(bit:not 'symbol)",
            "(bit:not \"string\")",
            "(bit:not :keyword)",
            "(bit:not '(s-expression))",
            "(bit:not {})",
            "(bit:not #())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(bit:not)",
            "(bit:not 1 2)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}