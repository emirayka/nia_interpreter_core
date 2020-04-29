use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::interpreter::Interpreter;

pub fn read_as_f64(interpreter: &Interpreter, value: Value) -> Result<f64, Error> {
    match value {
        Value::Float(float) => Ok(float),
        _ => Error::invalid_argument_error(
            "Expected int."
        ).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_int() {
        let mut interpreter = Interpreter::new();

        let value = Value::Float(3.0);
        let result = read_as_f64(
            &mut interpreter,
            value
        );

        assert_eq!(3.0, result.unwrap());
    }

    #[test]
    fn returns_invalid_argument_when_not_a_string_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let not_string_values = vec!(
            Value::Integer(1),
            Value::Boolean(true),
            Value::Boolean(false),
            interpreter.intern_string_value(String::from("test")),
            interpreter.intern_symbol_value("test"),
            interpreter.intern_keyword_value(String::from("test")),
            interpreter.make_cons_value(Value::Integer(1), Value::Integer(2)),
            interpreter.make_object_value(),
            interpreter.execute("#(+ %1 %2)").unwrap()
        );

        for not_string_value in not_string_values {
            let result = read_as_f64(
                &mut interpreter,
                not_string_value
            );
            assertion::assert_invalid_argument_error(&result);
        }
    }
}
