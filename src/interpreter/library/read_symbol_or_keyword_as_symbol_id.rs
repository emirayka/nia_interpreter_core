use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::symbol::SymbolId;

pub fn read_symbol_or_keyword_as_symbol_id(
    interpreter: &mut Interpreter,
    value: Value
) -> Result<SymbolId, Error> {
    let symbol_id = match value {
        Value::Symbol(symbol_id) => symbol_id,
        Value::Keyword(keyword_id) => {
            let keyword = interpreter.get_keyword(keyword_id)?;
            let keyword_name = keyword.get_name();
            let symbol_id = interpreter.intern(keyword_name);

            symbol_id
        },
        _ => return interpreter.make_invalid_argument_error(
            "Built-in function `object:get' takes only symbols or keywords as its second argument."
        ).into_result()
    };

    Ok(symbol_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_symbol_id_from_symbol() {
        let mut interpreter = Interpreter::new();

        let expected = interpreter.intern("test");
        let value = interpreter.intern_symbol_value("test");
        let result = read_symbol_or_keyword_as_symbol_id(
            &mut interpreter,
            value
        ).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn returns_correct_symbol_id_from_keyword() {
        let mut interpreter = Interpreter::new();

        let expected = interpreter.intern("test");
        let value = interpreter.intern_keyword_value(String::from("test"));
        let result = read_symbol_or_keyword_as_symbol_id(
            &mut interpreter,
            value
        ).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn returns_invalid_argument_when_not_a_symbol_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let invalid_values = vec!(
            Value::Integer(1),
            Value::Float(1.1),
            Value::Boolean(true),
            Value::Boolean(false),
            interpreter.intern_string_value(String::from("test")),
            interpreter.make_cons_value(Value::Integer(1), Value::Integer(2)),
            interpreter.make_object_value(),
            interpreter.execute("#(+ %1 %2)").unwrap()
        );

        for not_symbol_value in invalid_values {
            let result = read_symbol_or_keyword_as_symbol_id(
                &mut interpreter,
                not_symbol_value
            );
            assertion::assert_invalid_argument_error(&result);
        }
    }
}
