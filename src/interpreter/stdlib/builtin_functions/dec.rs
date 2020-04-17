use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

pub fn dec(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `dec' takes one argument exactly."
        ).into_result();
    }

    let mut values = values;

    match values.remove(0) {
        Value::Integer(int) => {
            match int.checked_sub(1) {
                Some(value) => Ok(Value::Integer(value)),
                _ => Error::overflow_error(
                    "Cannot decrement minimal value."
                ).into_result()
            }
        },
        _ => return Error::invalid_argument_error(
            "Built-in function `dec' takes one integer value."
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
            ("(dec 1)", Value::Integer(0)),
            ("(dec 2)", Value::Integer(1)),
            ("(dec 3)", Value::Integer(2)),
            ("(dec 4)", Value::Integer(3)),
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
            "(dec -9223372036854775808)",
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
            "(dec)",
            "(dec 1 2)",
            "(dec 1 2 3)"
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
            "(dec 1.1)",
            "(dec #t)",
            "(dec #f)",
            "(dec 'symbol)",
            "(dec \"string\")",
            "(dec :keyword)",
            "(dec '(s-expression))",
            "(dec {})",
            "(dec (function (lambda () 1)))",
            "(dec (function (macro () 1)))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
