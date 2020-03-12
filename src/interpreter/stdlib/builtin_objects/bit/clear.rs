use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn clear(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `bit:clear' takes two arguments exactly."
        ).into_result();
    }

    let mut values = values;

    let index = library::read_as_i64(
        interpreter,
        values.remove(0)
    )?;

    if index < 0 || index > 63 {
        return interpreter.make_invalid_argument_error(
            "Built-in function `bit:clear' takes value between [0; 64) as bit index."
        ).into_result()
    }

    let value = library::read_as_i64(
        interpreter,
        values.remove(0)
    )?;

    let result = (!(0x1 << index)) & value;

    Ok(Value::Integer(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn clears_bit() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(bit:clear 0 0)", "0"),
            ("(bit:clear 0 1)", "0"),
            ("(bit:clear 0 2)", "2"),
            ("(bit:clear 0 4)", "4"),

            ("(bit:clear 1 0)", "0"),
            ("(bit:clear 1 1)", "1"),
            ("(bit:clear 1 2)", "0"),
            ("(bit:clear 1 4)", "4"),

            ("(bit:clear 2 0)", "0"),
            ("(bit:clear 2 1)", "1"),
            ("(bit:clear 2 2)", "2"),
            ("(bit:clear 2 4)", "0"),
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
            "(bit:clear 1 1.1)",
            "(bit:clear 1 #t)",
            "(bit:clear 1 #f)",
            "(bit:clear 1 'symbol)",
            "(bit:clear 1 \"string\")",
            "(bit:clear 1 :keyword)",
            "(bit:clear 1 '(s-expression))",
            "(bit:clear 1 {})",
            "(bit:clear 1 #())",

            "(bit:clear 1.1 1)",
            "(bit:clear #t 1)",
            "(bit:clear #f 1)",
            "(bit:clear 'symbol 1)",
            "(bit:clear \"string\" 1)",
            "(bit:clear :keyword 1)",
            "(bit:clear '(s-expression) 1)",
            "(bit:clear {} 1)",
            "(bit:clear #() 1)",
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
            "(bit:clear)",
            "(bit:clear 1)",
            "(bit:clear 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
