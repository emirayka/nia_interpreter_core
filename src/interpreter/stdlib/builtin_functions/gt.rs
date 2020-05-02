use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

pub fn gt(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() < 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `>' takes two argument at least."
        ).into();
    }

    let mut values = values;
    let mut previous = values.remove(0);

    while values.len() > 0 {
        let current = values.remove(0);

        match (previous, current) {
            (Value::Integer(int1), Value::Integer(int2)) => {
                if int1 <= int2 {
                    return Ok(Value::Boolean(false))
                }
            },
            (Value::Integer(int1), Value::Float(float2)) => {
                if (int1 as f64) <= float2 {
                    return Ok(Value::Boolean(false))
                }
            },
            (Value::Float(float1), Value::Integer(int2)) => {
                if float1 <= (int2 as f64) {
                    return Ok(Value::Boolean(false))
                }
            },
            (Value::Float(float1), Value::Float(float2)) => {
                if float1 <= float2 {
                    return Ok(Value::Boolean(false))
                }
            },
            _ => return Error::invalid_argument_error(
                "Built-in function `>' takes only integer or float arguments"
            ).into()
        }

        previous = current;
    }

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_comparison_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(> 1 2)", "#f"),
            ("(> 1 2.0)", "#f"),
            ("(> 1.0 2)", "#f"),
            ("(> 1.0 2.0)", "#f"),

            ("(> 2 1)", "#t"),
            ("(> 2 1.0)", "#t"),
            ("(> 2.0 1)", "#t"),
            ("(> 2.0 1.0)", "#t"),

            ("(> 1 2 3)", "#f"),
            ("(> 3 2 1)", "#t"),

            ("(> 1 1 1)", "#f"),
            ("(> 1.0 1.0 1.0)", "#f"),
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
            "(>)",
            "(> 1)",
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
            "(> #t 1)",
            "(> #f 1)",
            "(> 'symbol 1)",
            "(> \"string\" 1)",
            "(> :keyword 1)",
            "(> '(s-expression) 1)",
            "(> {} 1)",
            "(> (function (lambda () 1)) 1)",
            "(> (function (macro () 1)) 1)",

            "(> 1 #t)",
            "(> 1 #f)",
            "(> 1 'symbol)",
            "(> 1 \"string\")",
            "(> 1 :keyword)",
            "(> 1 '(s-expression))",
            "(> 1 {})",
            "(> 1 (function (lambda () 1)))",
            "(> 1 (function (macro () 1)))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
