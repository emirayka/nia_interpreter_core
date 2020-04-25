use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::keyword::Keyword;

pub fn read_as_keyword(
    interpreter: &Interpreter,
    value: Value
) -> Result<&Keyword, Error> {
    let keyword = match value {
        Value::Keyword(keyword_id) => interpreter.get_keyword(keyword_id)?,
        _ => return Error::invalid_argument_error(
            "Expected keyword."
        ).into_result()
    };

    Ok(keyword)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_keyword() {
        let mut interpreter = Interpreter::new();

        let expected = "test";
        let keyword = interpreter.intern_keyword_value(String::from(expected));

        let result = read_as_keyword(
            &mut interpreter,
            keyword
        ).unwrap().get_name();

        assert_eq!(expected, result);
    }

    #[test]
    fn returns_invalid_argument_when_not_a_keyword_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let not_symbol_values = vec!(
            Value::Integer(1),
            Value::Float(1.1),
            Value::Boolean(true),
            Value::Boolean(false),
            interpreter.intern_string_value(String::from("test")),
            interpreter.intern_symbol_value("test"),
            interpreter.make_cons_value(Value::Integer(1), Value::Integer(2)),
            interpreter.make_object_value(),
            interpreter.execute("#(+ %1 %2)").unwrap()
        );

        for not_symbol_value in not_symbol_values {
            let result = read_as_keyword(
                &mut interpreter,
                not_symbol_value
            );
            assertion::assert_invalid_argument_error(&result);
        }
    }
}
