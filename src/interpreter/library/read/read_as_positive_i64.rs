use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn read_as_positive_i64(value: Value) -> Result<i64, Error> {
    match value {
        Value::Integer(int) if int > 0 => Ok(int),
        _ => Error::invalid_argument_error("Expected positive int.").into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_correct_int() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (Value::Integer(1), 1),
            (Value::Integer(2), 2),
            (Value::Integer(3), 3),
            (Value::Integer(4), 4),
        ];

        for spec in specs {
            let expected = spec.1;
            let result = read_as_positive_i64(spec.0).unwrap();

            nia_assert_equal(expected, result);
        }
    }

    #[test]
    fn returns_invalid_argument_when_not_a_positive_int_value_was_passed() {
        let mut interpreter = Interpreter::new();

        let not_string_values = vec![
            Value::Integer(-1),
            Value::Integer(0),
            Value::Float(1.1),
            Value::Boolean(true),
            Value::Boolean(false),
            interpreter.intern_string_value("test"),
            interpreter.intern_symbol_value("test"),
            interpreter.intern_keyword_value("test"),
            interpreter.make_cons_value(Value::Integer(1), Value::Integer(2)),
            interpreter.make_object_value(),
            interpreter
                .execute_in_main_environment("#(+ %1 %2)")
                .unwrap(),
        ];

        for not_string_value in not_string_values {
            let result = read_as_positive_i64(not_string_value);
            assertion::assert_invalid_argument_error(&result);
        }
    }
}
