use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::stdlib::builtin_functions::_lib::infect_builtin_function;
use crate::interpreter::environment::EnvironmentId;

fn rem(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Built-in function `%' must take exactly two arguments."
        ));
    }

    let mut values = values;

    let result = match (values.remove(0), values.remove(0)) {
        (Value::Integer(int1), Value::Integer(int2)) => match int2 {
            0 => return Err(Error::zero_division_error(
                interpreter,
                &format!("Can't compute the remainder of {} on {}.", int1, int2)
            )),
            _ => Value::Integer(int1 % int2),
        },
        (Value::Integer(int1), Value::Float(float2)) => if float2 == 0.0 {
            return Err(Error::zero_division_error(
                interpreter,
                &format!("Can't compute the remainder of {} on {}.", int1, float2)
            ));
        } else {
            Value::Float((int1 as f64) % float2)
        },
        (Value::Float(float1), Value::Integer(int2)) => match int2 {
            0 => return Err(Error::zero_division_error(
                interpreter,
                &format!("Can't compute the remainder of {} on {}.", float1, int2)
            )),
            _ => Value::Float(float1 % (int2 as f64)),
        },
        (Value::Float(float1), Value::Float(float2)) => if float2 == 0.0 {
            return Err(Error::zero_division_error(
                interpreter,
                &format!("Can't compute the remainder of {} on {}.", float1, float2)
            ));
        } else {
            Value::Float(float1 % float2)
        },
        _ => return Err(Error::invalid_argument(
            interpreter,
            "Built-in function `%' must take only integer or float values."
        ))
    };

    Ok(result)
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_builtin_function(interpreter, "%", rem)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_correct_integer_division() {
        let mut interpreter = Interpreter::new();

        assert_eq!(Value::Integer(2), interpreter.execute("(% 5 3)").unwrap());
    }

    #[test]
    fn returns_correct_float_division() {
        let mut interpreter = Interpreter::new();

        assert_eq!(Value::Float(2.0), interpreter.execute("(% 7 5.0)").unwrap());
        assert_eq!(Value::Float(2.0), interpreter.execute("(% 7.0 5)").unwrap());
        assert_eq!(Value::Float(2.0), interpreter.execute("(% 7.0 5.0)").unwrap());
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(%)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(% 1)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(% 1 2 3)");
        assertion::assert_invalid_argument_count_error(&result);
    }

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
            let incorrect_code = format!("(% 1 {})", incorrect_value);

            let result = interpreter.execute(&incorrect_code);

            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_zero_division_error_when_attempts_to_divide_on_zero() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(% 1 0)",
            "(% 1 0.0)",
            "(% 1.0 0)",
            "(% 1.0 0.0)",
        );

        for code in code_vector {
            let result = interpreter.execute(code);

            assertion::assert_zero_division_error(&result);
        }
    }
}
