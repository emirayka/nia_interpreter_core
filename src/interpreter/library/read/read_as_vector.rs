use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn read_as_vector(
    interpreter: &Interpreter,
    value: Value,
) -> Result<Vec<Value>, Error> {
    let vector = match value {
        Value::Cons(cons_id) => interpreter.list_to_vec(cons_id)?,
        Value::Symbol(symbol_id) => {
            if interpreter.symbol_is_nil(symbol_id)? {
                Vec::new()
            } else {
                return Error::invalid_argument_error("Expected list.").into();
            }
        },
        _ => return Error::invalid_argument_error("Expected list.").into(),
    };

    Ok(vector)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_correct_vector_representation() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            ("nil", vec![]),
            ("()", vec![]),
            ("'()", vec![]),
            ("'(1)", vec![Value::Integer(1)]),
            ("'(1 2)", vec![Value::Integer(1), Value::Integer(2)]),
            (
                "'(1 2 3)",
                vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)],
            ),
        ];

        for (code, expected) in specs {
            let value = interpreter.execute_in_main_environment(code).unwrap();
            let result = read_as_vector(&interpreter, value).unwrap();

            utils::assert_vectors_deep_equal(
                &mut interpreter,
                expected,
                result,
            );
        }
    }

    #[test]
    fn returns_invalid_argument_error_when_was_called_with_invalid_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "1",
            "1.1",
            "#t",
            "#f",
            "\"string\"",
            "'symbol",
            ":keyword",
            "{}",
            "#()",
        ];

        for code in code_vector {
            let value = interpreter.execute_in_main_environment(code).unwrap();
            let result = read_as_vector(&interpreter, value);

            utils::assert_invalid_argument_error(&result);
        }
    }
}
