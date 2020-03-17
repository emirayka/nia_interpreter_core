use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

pub fn and(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let values = values;
    let mut last_result = Value::Boolean(true);

    for value in values {
        let result = interpreter.execute_value(environment, value)?;

        if library::is_falsy(interpreter, result)? {
            return Ok(result)
        }

        last_result = result;
    }

    Ok(last_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn works_correctly() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(and)", "#t"),

            ("(and (logic:and #t #t) (logic:and #t #t))", "#t"),

            ("(and 1)", "1"),
            ("(and 1.1)", "1.1"),
            ("(and #t)", "#t"),
            ("(and #f)", "#f"),
            ("(and \"string\")", "\"string\""),
            ("(and 'symbol)", "'symbol"),
            ("(and :keyword)", ":keyword"),
            ("(and '(1 2))", "'(1 2)"),
            ("(and {})", "{}"),
            ("(and #())", "#()"),

            ("(and #t 1)", "1"),
            ("(and #t 1.1)", "1.1"),
            ("(and #t #t)", "#t"),
            ("(and #t #f)", "#f"),
            ("(and #t \"string\")", "\"string\""),
            ("(and #t 'symbol)", "'symbol"),
            ("(and #t :keyword)", ":keyword"),
            ("(and #t '(1 2))", "'(1 2)"),
            ("(and #t {})", "{}"),
            ("(and #t #())", "#()"),

            ("(and #f 1)", "#f"),
            ("(and #f 1.1)", "#f"),
            ("(and #f #t)", "#f"),
            ("(and #f #f)", "#f"),
            ("(and #f \"string\")", "#f"),
            ("(and #f 'symbol)", "#f"),
            ("(and #f :keyword)", "#f"),
            ("(and #f '(1 2))", "#f"),
            ("(and #f {})", "#f"),
            ("(and #f #())", "#f"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }
}
