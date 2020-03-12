use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

pub fn abs(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `math:abs' must take exactly one argument."
        ).into_result();
    }

    let mut values = values;

    match values.remove(0) {
        Value::Integer(int) => {
            if int < 0 {
                Ok(Value::Integer(-int))
            } else {
                Ok(Value::Integer(int))
            }
        },
        Value::Float(float) => {
            if float.is_sign_negative() {
                Ok(Value::Float(-float))
            } else {
                Ok(Value::Float(float))
            }
        },
        _ => return interpreter.make_invalid_argument_error(
            "Built-in function `math:abs' must take only integer or float values."
        ).into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_abs_of_the_value() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(math:abs 1)", Value::Integer(1)),
            ("(math:abs -1)", Value::Integer(1)),
            ("(math:abs 1.1)", Value::Float(1.1)),
            ("(math:abs -1.1)", Value::Float(1.1)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(math:abs #t)",
            "(math:abs #f)",
            "(math:abs 'symbol)",
            "(math:abs \"string\")",
            "(math:abs :keyword)",
            "(math:abs '(s-expression))",
            "(math:abs {})",
            "(math:abs (function (lambda () 1)))",
            "(math:abs (function (macro () 1)))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(math:abs)",
            "(math:abs 1 2)",
            "(math:abs 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

}
