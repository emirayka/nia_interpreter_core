use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::value::SymbolId;

pub fn read_as_symbol_id(
    interpreter: &Interpreter,
    value: Value
) -> Result<SymbolId, Error> {
    let symbol_id = match value {
        Value::Symbol(symbol_id) => symbol_id,
        _ => return Error::invalid_argument_error(
            "Expected symbol."
        ).into_result()
    };

    Ok(symbol_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_symbol_id() {
        let mut interpreter = Interpreter::new();

        let expected = interpreter.intern("test");
        let value = interpreter.intern_symbol_value("test");
        let result = read_as_symbol_id(
            &mut interpreter,
            value
        ).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn returns_invalid_argument_when_not_a_symbol_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let not_symbol_values = vec!(
            Value::Integer(1),
            Value::Float(1.1),
            Value::Boolean(true),
            Value::Boolean(false),
            interpreter.intern_string_value(String::from("test")),
            interpreter.intern_keyword_value(String::from("test")),
            interpreter.make_cons_value(Value::Integer(1), Value::Integer(2)),
            interpreter.make_object_value(),
            interpreter.execute("#(+ %1 %2)").unwrap()
        );

        for not_symbol_value in not_symbol_values {
            let result = read_as_symbol_id(
                &mut interpreter,
                not_symbol_value
            );
            assertion::assert_invalid_argument_error(&result);
        }
    }
}
