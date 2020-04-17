use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

pub fn ceil(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `math:ceil' must take exactly one argument."
        ).into_result();
    }

    let mut values = values;

    match values.remove(0) {
        Value::Integer(int) => Ok(Value::Integer(int)),
        Value::Float(float) => Ok(Value::Integer(float.ceil() as i64)),
        _ => return Error::invalid_argument_error(
            "Built-in function `math:ceil' must take only integer or float values."
        ).into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_the_integer_itself_if_it_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(math:ceil 3)", Value::Integer(3))
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn computes_a_ceiling_of_a_float_correctly() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(math:ceil 0.2)", Value::Integer(1)),
            ("(math:ceil 0.5)", Value::Integer(1)),
            ("(math:ceil 0.7)", Value::Integer(1)),
            ("(math:ceil 1.2)", Value::Integer(2)),
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
            "(math:ceil)",
            "(math:ceil 1 2)",
            "(math:ceil 1 2 3)"
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
             "(math:ceil #t)",
             "(math:ceil #f)",
             "(math:ceil 'symbol)",
             "(math:ceil \"string\")",
             "(math:ceil :keyword)",
             "(math:ceil '(s-expression))",
             "(math:ceil {})",
             "(math:ceil (function (lambda () 1)))",
             "(math:ceil (function (macro () 1)))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
