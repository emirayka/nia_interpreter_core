use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn check_value_is_string_or_nil(
    interpreter: &mut Interpreter,
    value: Value,
) -> Result<(), Error> {
    match value {
        Value::Symbol(symbol_id) => {
            if interpreter.symbol_is_not_nil(symbol_id)? {
                return Error::invalid_argument_error("Expected string or nil")
                    .into();
            }

            Ok(())
        }
        Value::String(_) => Ok(()),
        _ => Error::invalid_argument_error("Expected string or nil").into(),
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

        let specs = vec![
            interpreter.intern_string_value("test"),
            interpreter.intern_nil_symbol_value(),
        ];

        for spec in specs {
            nia_assert_equal(
                Ok(()),
                check_value_is_string_or_nil(&mut interpreter, spec),
            );
        }
    }

    #[test]
    fn returns_invalid_argument_when_not_a_string_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
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

        for spec in specs {
            let result = check_value_is_string_or_nil(&mut interpreter, spec);
            utils::assert_invalid_argument_error(&result);
        }
    }
}
