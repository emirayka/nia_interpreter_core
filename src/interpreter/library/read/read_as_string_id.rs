use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::StringId;
use crate::interpreter::value::Value;

pub fn read_as_string_id(value: Value) -> Result<StringId, Error> {
    let string_id = match value {
        Value::String(string_id) => string_id,
        _ => return Error::invalid_argument_error("Expected string.").into(),
    };

    Ok(string_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;
    use std::convert::TryInto;

    #[test]
    fn returns_correct_string() {
        let mut interpreter = Interpreter::new();

        let expected = interpreter.intern_string_id("test");
        let value = Value::from(expected);

        let result = read_as_string_id(value);

        nia_assert_equal(expected, result.unwrap());
    }

    #[test]
    fn returns_invalid_argument_when_not_a_string_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let not_string_values = vec![
            Value::Integer(1),
            Value::Float(1.1),
            Value::Boolean(true),
            Value::Boolean(false),
            interpreter.intern_symbol_value("test"),
            interpreter.intern_keyword_value("test"),
            interpreter.make_cons_value(Value::Integer(1), Value::Integer(2)),
            interpreter.make_object_value(),
            interpreter
                .execute_in_main_environment("#(+ %1 %2)")
                .unwrap(),
        ];

        for not_string_value in not_string_values {
            let result = read_as_string_id(not_string_value);
            assertion::assert_invalid_argument_error(&result);
        }
    }
}
