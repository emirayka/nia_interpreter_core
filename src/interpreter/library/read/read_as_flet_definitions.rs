use std::convert::TryInto;

use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::FunctionArguments;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn read_as_flet_definitions(
    interpreter: &mut Interpreter,
    value: Value,
) -> Result<Vec<(Value, FunctionArguments, Vec<Value>)>, Error> {
    let mut definitions = vec![];

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
                            return Error::generic_execution_error(
                                "If flet definition is a list, it must have 2 items at least."
                            ).into();
                        }

                        let name = vector.remove(0);

                        library::check_value_is_symbol(
                            name,
                        )?;

                        library::check_symbol_is_assignable(
                            interpreter,
                            name.try_into()?,
                        )?;

                        let arguments = library::read_as_arguments(
                            interpreter,
                            vector.remove(0),
                        )?;

                        let code = vector;

                        definitions.push((name, arguments, code))
                    }
                    _ => return Error::invalid_argument_error(
                        "Let definitions consist of assignable symbols or lists of structure `(symbol value)'."
                    ).into()
                }
            }

            cons_cells
        }
        Value::Symbol(symbol_id) => {
            if interpreter.symbol_is_nil(symbol_id)? {
                Vec::new()
            } else {
                return Error::invalid_argument_error("").into();
            }
        }
        _ => return Error::invalid_argument_error("").into(),
    };

    Ok(definitions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_vector_of_cons_cells_when_a_list_was_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (vec![], "nil"),
            (vec![], "'()"),
            (
                vec![(
                    interpreter.execute_in_main_environment("'a").unwrap(),
                    { FunctionArguments::new() },
                    vec![],
                )],
                "'((a ()))",
            ),
            (
                vec![(
                    interpreter.execute_in_main_environment("'a").unwrap(),
                    { FunctionArguments::new() },
                    vec![Value::Integer(1)],
                )],
                "'((a () 1))",
            ),
            (
                vec![
                    (
                        interpreter.execute_in_main_environment("'a").unwrap(),
                        { FunctionArguments::new() },
                        vec![],
                    ),
                    (
                        interpreter.execute_in_main_environment("'b").unwrap(),
                        { FunctionArguments::new() },
                        vec![],
                    ),
                ],
                "'((a ()) (b ()))",
            ),
            (
                vec![(
                    interpreter.execute_in_main_environment("'a").unwrap(),
                    {
                        let mut arguments = FunctionArguments::new();

                        arguments
                            .add_ordinary_argument(String::from("b"))
                            .unwrap();

                        arguments
                    },
                    vec![Value::Integer(1)],
                )],
                "'((a (b) 1))",
            ),
            (
                vec![(
                    interpreter.execute_in_main_environment("'a").unwrap(),
                    {
                        let mut arguments = FunctionArguments::new();

                        arguments
                            .add_optional_argument(
                                String::from("b"),
                                None,
                                None,
                            )
                            .unwrap();

                        arguments
                    },
                    vec![Value::Integer(1)],
                )],
                "'((a (#opt b) 1))",
            ),
            (
                vec![(
                    interpreter.execute_in_main_environment("'a").unwrap(),
                    {
                        let mut arguments = FunctionArguments::new();

                        arguments.add_rest_argument(String::from("b")).unwrap();

                        arguments
                    },
                    vec![Value::Integer(1)],
                )],
                "'((a (#rest b) 1))",
            ),
            (
                vec![(
                    interpreter.execute_in_main_environment("'a").unwrap(),
                    {
                        let mut arguments = FunctionArguments::new();

                        arguments
                            .add_key_argument(String::from("b"), None, None)
                            .unwrap();

                        arguments
                    },
                    vec![Value::Integer(1)],
                )],
                "'((a (#keys b) 1))",
            ),
        ];

        for spec in specs {
            let expected = spec.0;
            let value =
                interpreter.execute_in_main_environment(spec.1).unwrap();

            let result =
                read_as_flet_definitions(&mut interpreter, value).unwrap();

            nia_assert_equal(expected.len(), result.len());

            for (a, b) in expected.into_iter().zip(result.into_iter()) {
                utils::assert_deep_equal(&mut interpreter, a.0, b.0);

                nia_assert_equal(a.1, b.1);

                utils::assert_vectors_deep_equal(&mut interpreter, a.2, b.2);
            }
        }
    }

    #[test]
    fn returns_err_when_not_correct_lists_were_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec!["1", "'a", "'(a)", "'((a))", "'((1 ()))", "'((1 1))"];

        for spec in specs {
            let value = interpreter.execute_in_main_environment(spec).unwrap();

            let result = read_as_flet_definitions(&mut interpreter, value);

            nia_assert_is_err(&result);
        }
    }
}
