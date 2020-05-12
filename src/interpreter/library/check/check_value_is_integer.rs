use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn check_value_is_integer(value: Value) -> Result<(), Error> {
    match value {
        Value::Integer(_) => Ok(()),
        _ => Error::invalid_argument_error("Expected cons").into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::interpreter::value::ConsId;
    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_nothing_when_an_integer_was_passed() {
        let mut interpreter = Interpreter::new();

        let result = check_value_is_integer(Value::Integer(0)).unwrap();

        nia_assert_equal((), result);
    }

    #[test]
    fn returns_invalid_argument_when_not_a_cons_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            Value::Float(1.1),
            Value::Boolean(true),
            Value::Boolean(false),
            interpreter.intern_symbol_value("test"),
            interpreter.intern_nil_symbol_value(),
            interpreter.intern_string_value("test"),
            interpreter.intern_keyword_value("test"),
            interpreter.make_object_value(),
            interpreter.make_cons_value(Value::Integer(1), Value::Integer(2)),
            interpreter
                .execute_in_main_environment("#(+ %1 %2)")
                .unwrap(),
        ];

        for spec in specs {
            let result = check_value_is_integer(spec);
            utils::assert_invalid_argument_error(&result);
        }
    }
}
