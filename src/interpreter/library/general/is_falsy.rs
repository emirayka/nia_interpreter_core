use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn is_falsy(
    interpreter: &mut Interpreter,
    value: Value,
) -> Result<bool, Error> {
    match value {
        Value::Boolean(false) => Ok(true),
        Value::Symbol(symbol_id) => {
            let result = interpreter.symbol_is_nil(symbol_id)?;

            Ok(result)
        },
        _ => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn returns_false_when_value_is_truthy() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
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
        ];

        for (code, expected) in pairs {
            let value = interpreter.execute_in_main_environment(code).unwrap();
            let result = is_falsy(&mut interpreter, value).unwrap();

            nia_assert_equal(expected, result)
        }
    }

    #[test]
    fn returns_true_when_value_is_falsy() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![("#f", true), ("'()", true), ("nil", true)];

        for (code, expected) in pairs {
            let value = interpreter.execute_in_main_environment(code).unwrap();
            let result = is_falsy(&mut interpreter, value).unwrap();

            nia_assert_equal(expected, result)
        }
    }
}
