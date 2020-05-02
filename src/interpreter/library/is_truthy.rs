use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

pub fn is_truthy(
    interpreter: &mut Interpreter,
    value: Value
) -> Result<bool, Error> {
    match value {
        Value::Boolean(false) => Ok(false),
        Value::Symbol(symbol_id) => {
            let result = interpreter.symbol_is_not_nil(symbol_id)?;

            Ok(result)
        },
        _ => Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    #[test]
    fn returns_true_when_value_is_truthy() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("0", true),
            ("1", true),
            ("1.1", true),
            ("#t", true),
            ("\"\"", true),
            ("\"string\"", true),
            ("'symbol", true),
            (":keyword", true),
            ("'(1 2)", true),
            ("{}", true),
            ("{:a 1}", true),
            ("#()", true),
        );

        for (code, expected) in pairs {
            let value = interpreter.execute(code).unwrap();
            let result = is_truthy(&mut interpreter, value).unwrap();

            nia_assert_equal(expected, result)
        }
    }

    #[test]
    fn returns_false_when_value_is_falsy() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("#f", false),
            ("'()", false),
            ("nil", false),
        );

        for (code, expected) in pairs {
            let value = interpreter.execute(code).unwrap();
            let result = is_truthy(&mut interpreter, value).unwrap();

            nia_assert_equal(expected, result)
        }
    }
}
