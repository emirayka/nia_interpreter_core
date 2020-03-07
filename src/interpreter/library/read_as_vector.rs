use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

pub fn read_as_vector(interpreter: &Interpreter, value: Value) -> Result<Vec<Value>, Error> {
    let vector = match value {
        Value::Cons(cons_id) => interpreter.list_to_vec(cons_id)?,
        Value::Symbol(symbol_id) => {
            let symbol = interpreter.get_symbol(symbol_id)?;

            if symbol.is_nil() {
                Vec::new()
            } else {
                return interpreter.make_invalid_argument_error(
                    "Expected list."
                ).into_result()
            }
        },
        _ => return interpreter.make_invalid_argument_error(
            "Expected list."
        ).into_result()
    };

    Ok(vector)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_vector_representation() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("nil", vec!()),
            ("()", vec!()),
            ("'()", vec!()),
            ("'(1)", vec!(Value::Integer(1))),
            ("'(1 2)", vec!(Value::Integer(1), Value::Integer(2))),
            ("'(1 2 3)", vec!(Value::Integer(1), Value::Integer(2), Value::Integer(3))),
        );

        for (code, expected) in specs {
            let value = interpreter.execute(code).unwrap();
            let result = read_as_vector(&interpreter, value).unwrap();

            assertion::assert_vectors_deep_equal(
                &mut interpreter,
                expected,
                result
            );
        }
    }

    #[test]
    fn returns_invalid_argument_error_when_was_called_with_invalid_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            "\"string\"",
            "'symbol",
            ":keyword",
            "{}",
            "#()",
        );

        for code in code_vector {
            let value = interpreter.execute(code).unwrap();
            let result = read_as_vector(&interpreter, value);

            assertion::assert_invalid_argument_error(
                &result
            );
        }
    }
}
