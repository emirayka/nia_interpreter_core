use crate::interpreter::function::Arguments;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn read_as_flet_definitions(
    interpreter: &mut Interpreter,
    value: Value,
) -> Result<Vec<(Value, Arguments, Vec<Value>)>, Error> {
    let mut definitions = vec!();

    match value {
        Value::Cons(cons_id) => {
            let cons_cells = interpreter.list_to_vec(cons_id)?;

            for cons_cell in &cons_cells {
                match *cons_cell {
                    Value::Cons(cons_id) => {
                        let mut vector = interpreter.list_to_vec(
                            cons_id
                        )?;

                        if vector.len() < 2 {
                            return interpreter.make_generic_execution_error(
                                "If flet definition is a list, it must have 2 items at least."
                            ).into_result();
                        }

                        let name = vector.remove(0);

                        library::check_value_is_symbol(
                            interpreter,
                            name,
                        )?;

                        library::check_if_symbol_assignable(
                            interpreter,
                            name.as_symbol_id(),
                        )?;

                        let arguments = library::read_as_arguments(
                            interpreter,
                            vector.remove(0),
                        )?;

                        let code = vector;

                        definitions.push((name, arguments, code))
                    }
                    _ => return interpreter.make_invalid_argument_error(
                        "Let definitions consist of assignable symbols or lists of structure `(symbol value)'."
                    ).into_result()
                }
            }

            cons_cells
        }
        Value::Symbol(symbol_id) => {
            if interpreter.symbol_is_nil(symbol_id)? {
                Vec::new()
            } else {
                return interpreter.make_invalid_argument_error("")
                    .into_result();
            }
        }
        _ => return interpreter.make_invalid_argument_error("").into_result()
    };

    Ok(definitions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_vector_of_cons_cells_when_a_list_was_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            (
                vec!(),
                "nil"
            ),
            (
                vec!(),
                "'()"
            ),
            (
                vec!(
                    (
                        interpreter.execute("'a").unwrap(),
                        {
                            Arguments::new()
                        },
                        vec!(),
                    ),
                ),
                "'((a ()))"
            ),
            (
                vec!(
                    (
                        interpreter.execute("'a").unwrap(),
                        {
                            Arguments::new()
                        },
                        vec!(
                            Value::Integer(1)
                        ),
                    ),
                ),
                "'((a () 1))"
            ),
            (
                vec!(
                    (
                        interpreter.execute("'a").unwrap(),
                        {
                            Arguments::new()
                        },
                        vec!(),
                    ),
                    (
                        interpreter.execute("'b").unwrap(),
                        {
                            Arguments::new()
                        },
                        vec!(),
                    ),
                ),
                "'((a ()) (b ()))"
            ),
            (
                vec!(
                    (
                        interpreter.execute("'a").unwrap(),
                        {
                            let mut arguments = Arguments::new();

                            arguments.add_ordinary_argument(String::from("b"));

                            arguments
                        },
                        vec!(
                            Value::Integer(1)
                        ),
                    ),
                ),
                "'((a (b) 1))"
            ),
            (
                vec!(
                    (
                        interpreter.execute("'a").unwrap(),
                        {
                            let mut arguments = Arguments::new();

                            arguments.add_optional_argument(String::from("b"), None, None);

                            arguments
                        },
                        vec!(
                            Value::Integer(1)
                        ),
                    ),
                ),
                "'((a (#opt b) 1))"
            ),
            (
                vec!(
                    (
                        interpreter.execute("'a").unwrap(),
                        {
                            let mut arguments = Arguments::new();

                            arguments.add_rest_argument(String::from("b"));

                            arguments
                        },
                        vec!(
                            Value::Integer(1)
                        ),
                    ),
                ),
                "'((a (#rest b) 1))"
            ),
            (
                vec!(
                    (
                        interpreter.execute("'a").unwrap(),
                        {
                            let mut arguments = Arguments::new();

                            arguments.add_key_argument(String::from("b"), None, None);

                            arguments
                        },
                        vec!(
                            Value::Integer(1)
                        ),
                    ),
                ),
                "'((a (#keys b) 1))"
            ),
        );

        for spec in specs {
            let expected = spec.0;
            let value = interpreter.execute(spec.1).unwrap();

            let result = read_as_flet_definitions(
                &mut interpreter,
                value,
            ).unwrap();

            assert_eq!(expected.len(), result.len());

            for (a, b) in expected.into_iter().zip(result.into_iter()) {
                assertion::assert_deep_equal(
                    &mut interpreter,
                    a.0,
                    b.0
                );

                assert_eq!(a.1, b.1);

                assertion::assert_vectors_deep_equal(
                    &mut interpreter,
                    a.2,
                    b.2
                );
            }
        }
    }

    #[test]
    fn returns_err_when_not_correct_lists_were_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "1",
            "'a",
            "'(a)",
            "'((a))",
            "'((1 ()))",
            "'((1 1))",
        );

        for spec in specs {
            let value = interpreter.execute(spec).unwrap();

            let result = read_as_flet_definitions(
                &mut interpreter,
                value,
            );

            assertion::assert_is_error(&result);
        }
    }
}
