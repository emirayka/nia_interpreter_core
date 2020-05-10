use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn rem(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `%' must take exactly two arguments.",
        )
        .into();
    }

    let mut values = values;

    let result =
        match (values.remove(0), values.remove(0)) {
            (Value::Integer(int1), Value::Integer(int2)) => match int2 {
                0 => {
                    return Error::zero_division_error(&format!(
                        "Can't compute the remainder of {} on {}.",
                        int1, int2
                    ))
                    .into();
                },
                _ => Value::Integer(int1 % int2),
            },
            (Value::Integer(int1), Value::Float(float2)) => {
                if float2 == 0.0 {
                    return Error::zero_division_error(&format!(
                        "Can't compute the remainder of {} on {}.",
                        int1, float2
                    ))
                    .into();
                } else {
                    Value::Float((int1 as f64) % float2)
                }
            },
            (Value::Float(float1), Value::Integer(int2)) => match int2 {
                0 => {
                    return Error::zero_division_error(&format!(
                        "Can't compute the remainder of {} on {}.",
                        float1, int2
                    ))
                    .into();
                },
                _ => Value::Float(float1 % (int2 as f64)),
            },
            (Value::Float(float1), Value::Float(float2)) => {
                if float2 == 0.0 {
                    return Error::zero_division_error(&format!(
                        "Can't compute the remainder of {} on {}.",
                        float1, float2
                    ))
                    .into();
                } else {
                    Value::Float(float1 % float2)
                }
            },
            _ => return Error::invalid_argument_error(
                "Built-in function `%' must take only integer or float values.",
            )
            .into(),
        };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_correct_integer_division() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![("(% 5 3)", Value::Integer(2))];

        utils::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_correct_float_division() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(% 7 5.0)", Value::Float(2.0)),
            ("(% 7.0 5)", Value::Float(2.0)),
            ("(% 7.0 5.0)", Value::Float(2.0)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(%)", "(% 1 2 3)", "(% 1)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(% 1 #t)",
            "(% 1 #f)",
            "(% 1 'symbol)",
            "(% 1 \"string\")",
            "(% 1 :keyword)",
            "(% 1 '(s-expression))",
            "(% 1 {})",
            "(% 1 (function (lambda () 1)))",
            "(% 1 (function (macro () 1)))",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_zero_division_error_when_attempts_to_divide_on_zero() {
        let mut interpreter = Interpreter::new();

        let code_vector =
            vec!["(% 1 0)", "(% 1 0.0)", "(% 1.0 0)", "(% 1.0 0.0)"];

        utils::assert_results_are_zero_division_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
