use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn assert(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `assert' takes exactly one argument."
        ).into_result();
    }

    let mut values = values;
    let result = values.remove(0);

    match result {
        Value::Boolean(true) => {
            Ok(Value::Boolean(true))
        },
        Value::Boolean(false) => {
            interpreter.make_assertion_error("Assertion failed: ")
                .into_result()
        },
        _ => {
            interpreter.make_invalid_argument_error(
                "Built-in function `assert' takes exactly one boolean argument."
            ).into_result()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_true_when_assertion_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(assert #t)", "#t"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_assertion_error_when_assertion_was_not_passed() {
        let mut interpreter = Interpreter::new();

        let code = "(assert #f)";
        let result = interpreter.execute(code);

        assertion::assert_assertion_error(
            &result
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_called_with_a_value_that_is_not_boolean() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(assert 1)",
            "(assert 1.1)",
            "(assert \"string\")",
            "(assert 'symbol)",
            "(assert :keyword)",
            "(assert {})",
            "(assert '(1 2))",
            "(assert #())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
