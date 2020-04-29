use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::interpreter::Interpreter;

pub fn read_as_bool(interpreter: &Interpreter, value: Value) -> Result<bool, Error> {
    match value {
        Value::Boolean(bool) => Ok(bool),
        _ => Error::invalid_argument_error(
            "Expected boolean."
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

        let pairs = vec!(
            (Value::Boolean(true), true),
            (Value::Boolean(false), false),
        );

        for (value, expected) in pairs {
            let result = read_as_bool(
                &mut interpreter,
                value
            ).unwrap();

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn returns_invalid_argument_when_not_a_string_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let not_boolean_values = vec!(
            Value::Integer(1),
            Value::Float(1.1),
            interpreter.intern_string_value("test"),
            interpreter.intern_symbol_value("test"),
            interpreter.intern_keyword_value("test"),
            interpreter.make_cons_value(Value::Integer(1), Value::Integer(2)),
            interpreter.make_object_value(),
            interpreter.execute("#(+ %1 %2)").unwrap()
        );

        for not_boolean_value in not_boolean_values {
            let result = read_as_bool(
                &mut interpreter,
                not_boolean_value
            );
            assertion::assert_invalid_argument_error(&result);
        }
    }
}
