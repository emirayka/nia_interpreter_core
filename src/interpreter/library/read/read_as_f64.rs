use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn read_as_f64(value: Value) -> Result<f64, Error> {
    match value {
        Value::Float(float) => Ok(float),
        _ => Error::invalid_argument_error("Expected int.").into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_correct_float() {
        let mut interpreter = Interpreter::new();

        let value = Value::Float(3.0);
        let result = read_as_f64(value);

        nia_assert_fequal(3.0, result.unwrap());
    }

    #[test]
    fn returns_invalid_argument_when_not_a_string_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let not_string_values = vec![
            Value::Integer(1),
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
            let result = read_as_f64(not_string_value);
            utils::assert_invalid_argument_error(&result);
        }
    }
}
