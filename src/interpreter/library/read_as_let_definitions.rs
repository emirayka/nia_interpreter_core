use std::convert::TryInto;

use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn read_as_let_definitions(
    interpreter: &mut Interpreter,
    value: Value,
) -> Result<Vec<Value>, Error> {
    let definitions = match value {
        Value::Cons(cons_id) => {
            let cons_cells = interpreter.list_to_vec(cons_id)?;

            for cons_cell in &cons_cells {
                match *cons_cell {
                    Value::Cons(cons_id) => {
                        let mut vector = interpreter.list_to_vec(
                            cons_id
                        )?;

                        if vector.len() != 2 {
                            return Error::generic_execution_error(
                                "If let definition is a list, it must have 2 items exactly."
                            ).into();
                        }

                        let car = vector.remove(0);

                        library::check_value_is_symbol(
                            interpreter,
                            car,
                        )?;

                        let car_symbol_id = car.try_into()?;
                        library::check_if_symbol_assignable(
                            interpreter,
                            car_symbol_id,
                        )?;
                    },
                    Value::Symbol(symbol_id) => {
                        library::check_if_symbol_assignable(
                            interpreter,
                            symbol_id,
                        )?;
                    },
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
                return Error::invalid_argument_error("")
                    .into();
            }
        }
        _ => return Error::invalid_argument_error("").into()
    };

    Ok(definitions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_vector_of_cons_cells() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            (
                vec!(),
                interpreter.execute("'()").unwrap()
            ),
            (
                vec!(),
                interpreter.execute("'()").unwrap()
            ),
            (
                vec!(
                    interpreter.execute("'a").unwrap(),
                ),
                interpreter.execute("'(a)").unwrap()
            ),
            (
                vec!(
                    interpreter.execute("'(a 1)").unwrap(),
                ),
                interpreter.execute("'((a 1))").unwrap()
            ),
            (
                vec!(
                    interpreter.execute("'a").unwrap(),
                    interpreter.execute("'b").unwrap(),
                ),
                interpreter.execute("'(a b)").unwrap()
            ),
            (
                vec!(
                    interpreter.execute("'a").unwrap(),
                    interpreter.execute("'(b 2)").unwrap(),
                ),
                interpreter.execute("'(a (b 2))").unwrap()
            ),
            (
                vec!(
                    interpreter.execute("'(a 1)").unwrap(),
                    interpreter.execute("'b").unwrap(),
                ),
                interpreter.execute("'((a 1) b)").unwrap()
            ),
            (
                vec!(
                    interpreter.execute("'(a 1)").unwrap(),
                    interpreter.execute("'(b 2)").unwrap(),
                ),
                interpreter.execute("'((a 1) (b 2))").unwrap()
            ),
        );

        for spec in specs {
            let expected = spec.0;
            let result = read_as_let_definitions(
                &mut interpreter,
                spec.1,
            ).unwrap();

            assertion::assert_vectors_deep_equal(
                &mut interpreter,
                expected,
                result,
            );
        }
    }

    #[test]
    fn returns_err_when_not_correct_lists_were_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "'((a 1 2))",
            "'((1 1))",
            "'(1)",

            "'(nil)",
            "'(#opt)",
            "'(#rest)",
            "'(#keys)",

            "'((nil 1))",
            "'((#opt 1))",
            "'((#rest 1))",
            "'((#keys 1))",
        );

        for spec in specs {
            let value = interpreter.execute(spec).unwrap();

            let result = read_as_let_definitions(
                &mut interpreter,
                value,
            );

            nia_assert_is_err(&result);
        }

    }
}
