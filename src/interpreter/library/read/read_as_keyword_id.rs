use crate::interpreter::error::Error;
use crate::interpreter::value::KeywordId;
use crate::interpreter::value::Value;

pub fn read_as_keyword_id(value: Value) -> Result<KeywordId, Error> {
    let symbol_id = match value {
        Value::Keyword(keyword_id) => keyword_id,
        _ => return Error::invalid_argument_error("Expected keyword.").into(),
    };

    Ok(symbol_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    use crate::Interpreter;

    #[test]
    fn returns_correct_keyword_id() {
        let mut interpreter = Interpreter::new();

        let expected = interpreter.intern_keyword_id("test");

        let value = Value::Keyword(expected);

        let result = read_as_keyword_id(value).unwrap();

        nia_assert_equal(expected, result);
    }

    #[test]
    fn returns_invalid_argument_when_not_a_symbol_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let not_symbol_values = vec![
            Value::Integer(1),
            Value::Float(1.1),
            Value::Boolean(true),
            Value::Boolean(false),
            interpreter.intern_string_value("test"),
            interpreter.intern_symbol_value("test"),
            interpreter.make_cons_value(Value::Integer(1), Value::Integer(2)),
            interpreter.make_object_value(),
            interpreter
                .execute_in_main_environment("#(+ %1 %2)")
                .unwrap(),
        ];

        for not_symbol_value in not_symbol_values {
            let result = read_as_keyword_id(not_symbol_value);
            utils::assert_invalid_argument_error(&result);
        }
    }
}
