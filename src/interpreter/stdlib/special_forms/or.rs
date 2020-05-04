use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn or(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    let values = values;
    let mut last_result = Value::Boolean(false);

    for value in values {
        let result = interpreter.execute_value(environment, value)?;

        if library::is_truthy(interpreter, result)? {
            return Ok(result);
        }

        last_result = result;
    }

    Ok(last_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn works_correctly() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(or)", "#f"),
            ("(or (logic:and #f #t) (logic:and #f #f))", "#f"),
            ("(or 1)", "1"),
            ("(or 1.1)", "1.1"),
            ("(or #t)", "#t"),
            ("(or #f)", "#f"),
            ("(or \"string\")", "\"string\""),
            ("(or 'symbol)", "'symbol"),
            ("(or :keyword)", ":keyword"),
            ("(or '(1 2))", "'(1 2)"),
            ("(or {})", "{}"),
            ("(or #())", "#()"),
            ("(or #t 1)", "#t"),
            ("(or #t 1.1)", "#t"),
            ("(or #t #t)", "#t"),
            ("(or #t #f)", "#t"),
            ("(or #t \"string\")", "#t"),
            ("(or #t 'symbol)", "#t"),
            ("(or #t :keyword)", "#t"),
            ("(or #t '(1 2))", "#t"),
            ("(or #t {})", "#t"),
            ("(or #t #())", "#t"),
            ("(or #f 1)", "1"),
            ("(or #f 1.1)", "1.1"),
            ("(or #f #t)", "#t"),
            ("(or #f #f)", "#f"),
            ("(or #f \"string\")", "\"string\""),
            ("(or #f 'symbol)", "'symbol"),
            ("(or #f :keyword)", ":keyword"),
            ("(or #f '(1 2))", "'(1 2)"),
            ("(or #f {})", "{}"),
            ("(or #f #())", "#()"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }
}
