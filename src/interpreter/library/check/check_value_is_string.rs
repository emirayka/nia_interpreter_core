use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn check_value_is_string(value: Value) -> Result<(), Error> {
    match value {
        Value::String(_) => Ok(()),
        _ => Error::invalid_argument_error("Expected string").into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::interpreter::value::StringId;
    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_nothing_when_a_string_was_passed() {
        let mut interpreter = Interpreter::new();

        let result =
            check_value_is_string(Value::String(StringId::new(0))).unwrap();

        nia_assert_equal((), result);
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
            let result = check_value_is_string(not_string_value);
            utils::assert_invalid_argument_error(&result);
        }
    }
}
