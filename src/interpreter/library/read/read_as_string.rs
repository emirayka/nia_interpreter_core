use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn read_as_string(
    interpreter: &Interpreter,
    value: Value,
) -> Result<&String, Error> {
    let string_id = match value {
        Value::String(string_id) => string_id,
        _ => return Error::invalid_argument_error("Expected string.").into(),
    };

    let string = interpreter
        .get_string(string_id)
        .map_err(|err| Error::generic_execution_error_caused("", err))?;

    Ok(string.get_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_correct_string() {
        let mut interpreter = Interpreter::new();

        let value = interpreter.intern_string_value("test");
        let result = read_as_string(&mut interpreter, value);

        nia_assert_equal("test", result.unwrap());
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
            let result = read_as_string(&mut interpreter, not_string_value);
            utils::assert_invalid_argument_error(&result);
        }
    }
}
