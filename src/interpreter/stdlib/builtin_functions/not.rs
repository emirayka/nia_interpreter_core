use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

pub fn not(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `not' takes exactly one argument."
        ).into_result();
    }

    let mut values = values;

    if library::is_truthy(interpreter, values.remove(0))? {
        Ok(Value::Boolean(false))
    } else {
        Ok(Value::Boolean(true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn works_correctly() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(not #f)", "#t"),

            ("(not 1)", "#f"),
            ("(not 1.1)", "#f"),
            ("(not #t)", "#f"),
            ("(not \"string\")", "#f"),
            ("(not 'symbol)", "#f"),
            ("(not :keyword)", "#f"),
            ("(not '(1 2))", "#f"),
            ("(not {})", "#f"),
            ("(not #())", "#f"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(not)",
            "(not 1 2)",
            "(not 1 2 3)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
