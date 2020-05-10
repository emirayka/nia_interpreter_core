use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::SymbolId;
use crate::interpreter::value::Value;

pub fn read_keyword_or_symbol_as_symbol_id(
    interpreter: &mut Interpreter,
    value: Value,
) -> Result<SymbolId, Error> {
    let symbol_id = match value {
        Value::Keyword(keyword_id) => {
            let keyword_name =
                interpreter.get_keyword(keyword_id)?.get_name().clone();

            let symbol_id = interpreter.intern_symbol_id(&keyword_name);

            symbol_id
        },
        Value::Symbol(symbol_id) => symbol_id,
        _ => {
            return Error::invalid_argument_error(
                "Expected keyword or symbol.",
            )
            .into();
        },
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

    #[test]
    fn returns_correct_symbol_id_from_symbol() {
        let mut interpreter = Interpreter::new();

        let expected = interpreter.intern_symbol_id("test");
        let value = interpreter.intern_symbol_value("test");
        let result =
            read_keyword_or_symbol_as_symbol_id(&mut interpreter, value)
                .unwrap();

        nia_assert_equal(expected, result);
    }

    #[test]
    fn returns_correct_symbol_id_from_keyword() {
        let mut interpreter = Interpreter::new();

        let expected = interpreter.intern_symbol_id("test");
        let value = interpreter.intern_keyword_value("test");
        let result =
            read_keyword_or_symbol_as_symbol_id(&mut interpreter, value)
                .unwrap();

        nia_assert_equal(expected, result);
    }

    #[test]
    fn returns_invalid_argument_when_not_a_symbol_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let invalid_values = vec![
            Value::Integer(1),
            Value::Float(1.1),
            Value::Boolean(true),
            Value::Boolean(false),
            interpreter.intern_string_value("test"),
            interpreter.make_cons_value(Value::Integer(1), Value::Integer(2)),
            interpreter.make_object_value(),
            interpreter
                .execute_in_main_environment("#(+ %1 %2)")
                .unwrap(),
        ];

        for not_symbol_value in invalid_values {
            let result = read_keyword_or_symbol_as_symbol_id(
                &mut interpreter,
                not_symbol_value,
            );
            utils::assert_invalid_argument_error(&result);
        }
    }
}
