use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

pub fn inc(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `inc' takes one argument exactly."
        ).into_result();
    }

    let mut values = values;

    match values.remove(0) {
        Value::Integer(int) => {
            match int.checked_add(1) {
                Some(value) => Ok(Value::Integer(value)),
                _ => interpreter.make_overflow_error(
                    "Cannot increment maximal value."
                ).into_result()
            }
        },
        _ => return interpreter.make_invalid_argument_error(
            "Built-in function `inc' takes one integer value."
        ).into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn computes_a_ceiling_of_a_float_correctly() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(inc 1)", Value::Integer(2)),
            ("(inc 2)", Value::Integer(3)),
            ("(inc 3)", Value::Integer(4)),
            ("(inc 4)", Value::Integer(5)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn overflows_correctly() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(inc 9223372036854775807)",
        );

        assertion::assert_results_are_overflow_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(inc)",
            "(inc 1 2)",
            "(inc 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(inc 1.1)",
            "(inc #t)",
            "(inc #f)",
            "(inc 'symbol)",
            "(inc \"string\")",
            "(inc :keyword)",
            "(inc '(s-expression))",
            "(inc {})",
            "(inc (function (lambda () 1)))",
            "(inc (function (macro () 1)))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}