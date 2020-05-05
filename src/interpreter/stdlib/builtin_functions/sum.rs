use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn sum(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() < 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `+' must take at least two arguments",
        )
        .into();
    }

    let mut result = Value::Integer(0);

    for value in values {
        result =
            match (value, result) {
                (Value::Integer(int1), Value::Integer(int2)) => {
                    match int1.checked_add(int2) {
                        Some(int_result) => Value::Integer(int_result),
                        None => {
                            return Error::overflow_error(&format!(
                                "Attempt to add values {} {} leads to overflow",
                                int1, int2
                            ))
                            .into();
                        },
                    }
                },
                (Value::Integer(int1), Value::Float(float2)) => {
                    Value::Float((int1 as f64) + float2)
                },
                (Value::Float(float1), Value::Integer(int2)) => {
                    Value::Float(float1 + (int2 as f64))
                },
                (Value::Float(float1), Value::Float(float2)) => {
                    Value::Float(float1 + float2)
                },
                _ => return Error::invalid_argument_error(
                    "Built-in function `+' must take only integers or float.",
                )
                .into(),
            };
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_correct_integer_sum() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![("(+ 1 2)", Value::Integer(3))];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_correct_float_sum() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(+ 1 2.0)", Value::Float(3.0)),
            ("(+ 1.0 2)", Value::Float(3.0)),
            ("(+ 1.0 2.0)", Value::Float(3.0)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn is_variadic() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(+ 1 2)", Value::Integer(3)),
            ("(+ 1 2 3)", Value::Integer(6)),
            ("(+ 1 2 3 4)", Value::Integer(10)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn converts_to_float_if_any_was_present() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(+ 1 2 3.0)", Value::Float(6.0)),
            ("(+ 1.0 2 3)", Value::Float(6.0)),
            ("(+ 1 2.0 3)", Value::Float(6.0)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(+)", "(+ 1)"];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(+ 1 #t)",
            "(+ 1 #f)",
            "(+ 1 'symbol)",
            "(+ 1 \"string\")",
            "(+ 1 :keyword)",
            "(+ 1 '(s-expression))",
            "(+ 1 {})",
            "(+ 1 (function (lambda () 1)))",
            "(+ 1 (function (macro () 1)))",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_overflow_error_when_an_overflow_occurred() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(+ 9223372036854775800 5 5)",
            "(+ 5 9223372036854775800 5)",
            "(+ 9223372036854775800 5 5)",
        ];

        assertion::assert_results_are_overflow_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
