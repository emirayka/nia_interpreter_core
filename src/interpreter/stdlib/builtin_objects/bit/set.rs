use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn set(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `bit:set' takes two arguments exactly."
        ).into_result();
    }

    let mut values = values;

    let index = library::read_as_i64(
        interpreter,
        values.remove(0)
    )?;

    if index < 0 || index > 63 {
        return Error::invalid_argument_error(
            "Built-in function `bit:set' takes value between [0; 64) as bit index."
        ).into_result()
    }

    let value = library::read_as_i64(
        interpreter,
        values.remove(0)
    )?;

    let result = (0x1 << index) | value;

    Ok(Value::Integer(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn sets_bit() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(bit:set 0 0)", "1"),
            ("(bit:set 0 1)", "1"),
            ("(bit:set 0 2)", "3"),
            ("(bit:set 0 4)", "5"),

            ("(bit:set 1 0)", "2"),
            ("(bit:set 1 1)", "3"),
            ("(bit:set 1 2)", "2"),
            ("(bit:set 1 4)", "6"),

            ("(bit:set 2 0)", "4"),
            ("(bit:set 2 1)", "5"),
            ("(bit:set 2 2)", "6"),
            ("(bit:set 2 4)", "4"),
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
            "(bit:set 1 1.1)",
            "(bit:set 1 #t)",
            "(bit:set 1 #f)",
            "(bit:set 1 'symbol)",
            "(bit:set 1 \"string\")",
            "(bit:set 1 :keyword)",
            "(bit:set 1 '(s-expression))",
            "(bit:set 1 {})",
            "(bit:set 1 #())",

            "(bit:set 1.1 1)",
            "(bit:set #t 1)",
            "(bit:set #f 1)",
            "(bit:set 'symbol 1)",
            "(bit:set \"string\" 1)",
            "(bit:set :keyword 1)",
            "(bit:set '(s-expression) 1)",
            "(bit:set {} 1)",
            "(bit:set #() 1)",
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
            "(bit:set)",
            "(bit:set 1)",
            "(bit:set 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
