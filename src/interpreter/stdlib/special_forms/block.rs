use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn block(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let values = values;
    let mut results = Vec::new();

    for value in values {
        let result = interpreter.execute_value(environment, value)?;

        results.push(result);
    }

    Ok(interpreter.vec_to_list(results))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    // todo: ensure this test is fine
    #[test]
    fn returns_list_of_execution_results() {
        let mut interpreter = Interpreter::new();
        let nil = interpreter.intern_nil_symbol_value();

        assert_eq!(interpreter.intern_nil_symbol_value(), interpreter.execute("(block)").unwrap());

        let expected = interpreter.make_cons_value(
            Value::Integer(1),
            nil
        );
        let result = interpreter.execute("(block 1)").unwrap();

        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result
        );

        let cdr = interpreter.make_cons_value(
            Value::Integer(2),
            nil
        );

        let expected = interpreter.make_cons_value(
            Value::Integer(1),
            cdr,
        );

        let result = interpreter.execute("(block 1 2)").unwrap();

        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result
        );

        let cdr = interpreter.make_cons_value(
            Value::Integer(3),
            nil
        );

        let cdr = interpreter.make_cons_value(
            Value::Integer(2),
            cdr
        );

        let expected = interpreter.make_cons_value(
            Value::Integer(1),
            cdr
        );

        let result = interpreter.execute("(block 1 2 3)").unwrap();

        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result
        );
    }
}
