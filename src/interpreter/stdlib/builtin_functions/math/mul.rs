use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::stdlib::builtin_functions::_lib::infect_builtin_function;

fn mul(
    interpreter: &mut Interpreter,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() < 2 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Built-in function `*' must take at least two arguments"
        ));
    }

    let mut result = Value::Integer(1);

    for value in values {
        result = match (value, result) {
            (Value::Integer(int1), Value::Integer(int2)) => match int1.checked_mul(int2) {
                Some(int_result) => Value::Integer(int_result),
                None => return Err(Error::overflow_error(
                    interpreter,
                    &format!("Attempt to multiply values {} {} leads to overflow", int1, int2)
                ))
            },
            (Value::Integer(int1), Value::Float(float2)) => Value::Float((int1 as f64) * float2),
            (Value::Float(float1), Value::Integer(int2)) => Value::Float(float1 * (int2 as f64)),
            (Value::Float(float1), Value::Float(float2)) => Value::Float(float1 * float2),
            _ => return Err(Error::invalid_argument(
                interpreter,
                "Built-in function `*' must take only integers or float."
            ))
        };
    }

    Ok(result)
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_builtin_function(interpreter, "*", mul)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::error::assertion;

    #[test]
    fn returns_correct_integer_multiplication() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        assert_eq!(Value::Integer(2), interpreter.execute("(* 1 2)").unwrap());
    }

    #[test]
    fn returns_correct_float_multiplication() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        assert_eq!(Value::Float(2.0), interpreter.execute("(* 1 2.0)").unwrap());
        assert_eq!(Value::Float(2.0), interpreter.execute("(* 1.0 2)").unwrap());
        assert_eq!(Value::Float(2.0), interpreter.execute("(* 1.0 2.0)").unwrap());
    }

    #[test]
    fn is_variadic() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        assert_eq!(Value::Integer(2), interpreter.execute("(* 1 2)").unwrap());
        assert_eq!(Value::Integer(6), interpreter.execute("(* 1 2 3)").unwrap());
        assert_eq!(Value::Integer(24), interpreter.execute("(* 1 2 3 4)").unwrap());
    }

    #[test]
    fn converts_to_float_if_any_was_present() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        assert_eq!(Value::Float(6.0), interpreter.execute("(* 1 2 3.0)").unwrap());
        assert_eq!(Value::Float(6.0), interpreter.execute("(* 1.0 2 3)").unwrap());
        assert_eq!(Value::Float(6.0), interpreter.execute("(* 1 2.0 3)").unwrap());
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        let result = interpreter.execute("(*)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(* 1)");
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
            "(function (lambda () 1))",
            "(function (macro () 1))",
        );

        for incorrect_value in incorrect_values {
            let incorrect_code = format!("(* 1 {})", incorrect_value);

            let result = interpreter.execute(&incorrect_code);

            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_overflow_error_when_an_overflow_occurred() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        let code_vector = vec!(
            "(* 922337203685477580 10 10)",
            "(* 10 922337203685477580 10)",
            "(* 922337203685477580 10 10)",
        );

        for code in code_vector {
            let result = interpreter.execute(code);

            assertion::assert_overflow_error(&result);
        }
    }
}