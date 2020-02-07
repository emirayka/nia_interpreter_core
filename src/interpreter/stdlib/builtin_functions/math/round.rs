use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::stdlib::builtin_functions::_lib::infect_builtin_function;

fn round(
    interpreter: &mut Interpreter,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Built-in function `round' must take exactly one argument."
        ));
    }

    let mut values = values;

    match values.remove(0) {
        Value::Integer(int) => Ok(Value::Integer(int)),
        Value::Float(float) => Ok(Value::Integer(float.round() as i64)),
        _ => return Err(Error::invalid_argument(
            interpreter,
            "Built-in function `round' must take only integer or float values."
        ))
    }
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_builtin_function(interpreter, "round", round)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::error::assertion;

    #[test]
    fn returns_integer_itself_if_it_was_passed() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        assert_eq!(Value::Integer(3), interpreter.execute("(round 3)").unwrap());
    }

    #[test]
    fn rounds_floats_correctly() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        assert_eq!(Value::Integer(0), interpreter.execute("(round 0.2)").unwrap());
        assert_eq!(Value::Integer(1), interpreter.execute("(round 0.5)").unwrap());
        assert_eq!(Value::Integer(1), interpreter.execute("(round 0.7)").unwrap());
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        let result = interpreter.execute("(round)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(round 1 2)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(round 1 2 3)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::raw();

        crate::interpreter::stdlib::special_forms::infect(&mut interpreter).unwrap();
        infect(&mut interpreter).unwrap();

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
            let incorrect_code = format!("(round {})", incorrect_value);

            let result = interpreter.execute(&incorrect_code);

            assertion::assert_invalid_argument_error(&result);
        }
    }
}
