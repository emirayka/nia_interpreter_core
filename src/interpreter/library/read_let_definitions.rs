use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

pub fn read_let_definitions(
    interpreter: &mut Interpreter,
    value: Value
) -> Result<Vec<Value>, Error> {
    let definitions = match value {
        Value::Cons(cons_id) => {
            interpreter.list_to_vec(cons_id)
                .map_err(|err| interpreter.make_generic_execution_error_caused(
                    "",
                    err
                ))?
        },
        Value::Symbol(symbol_id) => {
            let symbol = match interpreter.get_symbol(symbol_id) {
                Ok(symbol) => symbol,
                Err(error) => return interpreter.make_generic_execution_error_caused(
                    "",
                    error
                ).into_result()
            };

            if symbol.is_nil() {
                Vec::new()
            } else {
                return interpreter.make_invalid_argument_error("")
                    .into_result();
            }
        }
        _ => return interpreter.make_invalid_argument_error("").into_result()
    };

    for definition in &definitions {
        match definition {
            Value::Cons(_) => {},
            Value::Symbol(symbol_id) => {
                let symbol = match interpreter.get_symbol(*symbol_id) {
                    Ok(symbol) => symbol,
                    Err(error) => return interpreter.make_generic_execution_error_caused(
                        "",
                        error
                    ).into_result()
                };

                if symbol.is_nil() {
                    return interpreter.make_invalid_argument_error("").into_result()
                }
            }
            _ => return interpreter.make_invalid_argument_error("").into_result()
        }
    };

    Ok(definitions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    // todo: ensure this test is fine
    #[test]
    fn returns_empty_vector_when_nil_was_provided() {
        let mut interpreter = Interpreter::new();
        let nil = interpreter.intern_nil_symbol_value();

        let result = read_let_definitions(
            &mut interpreter,
            nil
        );

        let expected: Vec<Value> = Vec::new();

        assert_eq!(expected, result.unwrap());
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_vector_of_cons_cells_when_a_list_was_provided() {
        let mut interpreter = Interpreter::new();

        let mut expected = vec!();
        expected.push(interpreter.execute("(quote (1 2))").unwrap());
        expected.push(interpreter.execute("(quote (1 2))").unwrap());

        let value = interpreter.execute("(quote ((1 2) (1 2)))").unwrap();
        let result = read_let_definitions(
            &mut interpreter,
            value
        ).unwrap();

        assertion::assert_vectors_deep_equal(
            &mut interpreter,
            expected,
            result
        );
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_err_when_neither_a_cons_nor_symbol_were_provided() {
        let mut interpreter = Interpreter::new();

        let value = interpreter.execute("(quote ((1 2) 1))").unwrap();

        let result = read_let_definitions(
            &mut interpreter,
            value
        );

        assertion::assert_is_error(&result);
    }
}
