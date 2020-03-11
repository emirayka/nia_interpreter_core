use crate::interpreter::cons::ConsId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

pub fn is_falsy(
    interpreter: &mut Interpreter,
    value: Value
) -> Result<bool, Error> {
    match value {
        Value::Boolean(false) => Ok(true),
        Value::Symbol(symbol_id) => {
            let symbol = interpreter.get_symbol(
                symbol_id
            )?;

            Ok(symbol.is_nil())
        },
        _ => Ok(false)
    }
}

mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_false_when_value_is_truthy() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("0", false),
            ("1", false),
            ("1.1", false),
            ("#t", false),
            ("\"\"", false),
            ("\"string\"", false),
            ("'symbol", false),
            (":keyword", false),
            ("'(1 2)", false),
            ("{}", false),
            ("{:a 1}", false),
            ("#()", false),
        );

        for (code, expected) in pairs {
            let value = interpreter.execute(code).unwrap();
            let result = is_falsy(&mut interpreter, value).unwrap();

            assert_eq!(expected, result)
        }
    }

    #[test]
    fn returns_true_when_value_is_falsy() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("#f", true),
            ("'()", true),
            ("nil", true),
        );

        for (code, expected) in pairs {
            let value = interpreter.execute(code).unwrap();
            let result = is_falsy(&mut interpreter, value).unwrap();

            assert_eq!(expected, result)
        }
    }
}
