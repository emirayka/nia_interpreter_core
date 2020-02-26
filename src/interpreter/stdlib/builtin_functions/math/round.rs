use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn round(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `round' must take exactly one argument."
        ).into_result();
    }

    let mut values = values;

    match values.remove(0) {
        Value::Integer(int) => Ok(Value::Integer(int)),
        Value::Float(float) => Ok(Value::Integer(float.round() as i64)),
        _ => return interpreter.make_invalid_argument_error(
            "Built-in function `round' must take only integer or float values."
        ).into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_the_integer_itself_if_it_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(round 3)", Value::Integer(3))
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn computes_a_correct_round_of_a_float_correctly() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(round 0.2)", Value::Integer(0)),
            ("(round 0.5)", Value::Integer(1)),
            ("(round 0.7)", Value::Integer(1)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(round)",
            "(round 1 2)",
            "(round 1 2 3)",
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
            "(round #t)",
            "(round #f)",
            "(round 'symbol)",
            "(round \"string\")",
            "(round :keyword)",
            "(round '(s-expression))",
            "(round {})",
            "(round (function (lambda () 1)))",
            "(round (function (macro () 1)))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        )
    }
}
