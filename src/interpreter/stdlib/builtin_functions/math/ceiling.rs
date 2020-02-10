use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

pub fn ceiling(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Built-in function `ceiling' must take exactly one argument."
        ));
    }

    let mut values = values;

    match values.remove(0) {
        Value::Integer(int) => Ok(Value::Integer(int)),
        Value::Float(float) => Ok(Value::Integer(float.ceil() as i64)),
        _ => return Err(Error::invalid_argument(
            interpreter,
            "Built-in function `ceiling' must take only integer or float values."
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_the_integer_itself_if_it_was_passed() {
        let mut interpreter = Interpreter::new();

        assert_eq!(Value::Integer(3), interpreter.execute("(ceiling 3)").unwrap());
    }

    #[test]
    fn computes_a_ceiling_of_a_float_correctly() {
        let mut interpreter = Interpreter::new();

        assert_eq!(Value::Integer(1), interpreter.execute("(ceiling 0.2)").unwrap());
        assert_eq!(Value::Integer(1), interpreter.execute("(ceiling 0.5)").unwrap());
        assert_eq!(Value::Integer(1), interpreter.execute("(ceiling 0.7)").unwrap());
        assert_eq!(Value::Integer(2), interpreter.execute("(ceiling 1.2)").unwrap());
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(ceiling)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(ceiling 1 2)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(ceiling 1 2 3)");
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
            let incorrect_code = format!("(ceiling {})", incorrect_value);

            let result = interpreter.execute(&incorrect_code);

            assertion::assert_invalid_argument_error(&result);
        }
    }
}
