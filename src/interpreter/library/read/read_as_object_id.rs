use crate::interpreter::error::Error;
use crate::interpreter::value::ObjectId;
use crate::interpreter::value::Value;

pub fn read_as_object_id(value: Value) -> Result<ObjectId, Error> {
    let object_id = match value {
        Value::Object(object_id) => object_id,
        _ => {
            return Error::invalid_argument_error("Expected an object.").into();
        }
    };

    Ok(object_id)
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
    fn returns_correct_object_id() {
        let expected = ObjectId::new(0);

        let value = Value::Object(expected);
        let result = read_as_object_id(value).unwrap();

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
            interpreter.intern_symbol_value("test"),
            interpreter.intern_string_value("test"),
            interpreter.intern_keyword_value("test"),
            interpreter.make_cons_value(Value::Integer(1), Value::Integer(2)),
            interpreter
                .execute_in_main_environment("#(+ %1 %2)")
                .unwrap(),
        ];

        for not_symbol_value in not_symbol_values {
            let result = read_as_object_id(not_symbol_value);
            utils::assert_invalid_argument_error(&result);
        }
    }
}
