use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn sub(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `-' must take exactly two argument"
        ).into_result();
    }

    let mut values = values;

    let result = match (values.remove(0), values.remove(0)) {
        (Value::Integer(int1), Value::Integer(int2)) => match int1.checked_sub(int2) {
            Some(int_result) => Value::Integer(int_result),
            None => return interpreter.make_overflow_error(
                &format!("Attempt to subtract values {} {} leads to overflow", int1, int2)
            ).into_result()
        },
        (Value::Integer(int1), Value::Float(float2)) => Value::Float((int1 as f64) - float2),
        (Value::Float(float1), Value::Integer(int2)) => Value::Float(float1 - (int2 as f64)),
        (Value::Float(float1), Value::Float(float2)) => Value::Float(float1 - float2),
        _ => return interpreter.make_invalid_argument_error(
            "Built-in function `-' must take only integers or float."
        ).into_result()
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    // todo: ensure this test is fine
    #[test]
    fn returns_correct_subtraction_of_two_integers() {
        let mut interpreter = Interpreter::new();

        assert_eq!(Value::Integer(-1), interpreter.execute("(- 1 2)").unwrap());
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_correct_float_subtraction() {
        let mut interpreter = Interpreter::new();

        assert_eq!(Value::Float(-1.0), interpreter.execute("(- 1 2.0)").unwrap());
        assert_eq!(Value::Float(-1.0), interpreter.execute("(- 1.0 2)").unwrap());
        assert_eq!(Value::Float(-1.0), interpreter.execute("(- 1.0 2.0)").unwrap());
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(-)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(- 1)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(- 1 2 3)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let incorrect_values = vec!(
            "#t",
            "#f",
            "'symbol",
            "\"string\"",
            ":keyword",
            "'(s-expression)",
            "{}",
            "(function (lambda () 1))",
            "(function (macro () 1))",
        );

        for incorrect_value in incorrect_values {
            let incorrect_code = format!("(- 1 {})", incorrect_value);

            let result = interpreter.execute(&incorrect_code);

            assertion::assert_invalid_argument_error(&result);
        }
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_overflow_error_when_an_overflow_occurred() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(- 9223372036854775800 -10)",
            "(- 10 -9223372036854775800)",
            "(- -10 9223372036854775800)",
            "(- -9223372036854775800 10)",
        );

        for code in code_vector {
            let result = interpreter.execute(code);

            assertion::assert_overflow_error(&result);
        }
    }
}
