use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::cons::cons_arena::ConsId;
use crate::interpreter::stdlib::_lib;

fn parse_catch_clauses(interpreter: &mut Interpreter, clauses: Vec<Value>) -> Result<Vec<ConsId>, Error> {
    let mut catch_clauses = Vec::new();

    for clause in clauses {
        match clause {
            Value::Cons(cons_id) => {
                let car = interpreter.get_car(cons_id)
                    .map_err(|err| interpreter.make_generic_execution_error_caused(
                        "",
                        err
                    ))?;
                
                match car {
                    Value::Symbol(symbol_id) => {
                        let symbol = interpreter.get_symbol(symbol_id)?;

                        if symbol.get_name() == "catch" {
                            catch_clauses.push(cons_id)
                        }
                    },
                    _ => return interpreter.make_invalid_argument_error(
                        "The first item of catch clauses must be a catch symbol."
                    ).into_result(),
                }
            }
            _ => return interpreter.make_invalid_argument_error(
                "The clauses of special form `try' must be lists."
            ).into_result()
        }
    }

    for clause in &catch_clauses {
        interpreter.get_cddr(*clause)
            .map_err(|_| interpreter.make_invalid_argument_error(
                "The clauses of special form `try' must be lists with two items at least."
            ))?;
    }

    Ok(catch_clauses)
}

pub fn _try(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() < 2 {
        return interpreter.make_invalid_argument_count_error(
            "Special form `try' must take at least two arguments"
        ).into_result();
    }

    let mut values = values;

    let try_code = values.remove(0);
    let clauses = values;

    let catch_clauses = parse_catch_clauses(interpreter, clauses)?;
    let try_result = interpreter.execute_value(environment_id, try_code);

    match try_result {
        Ok(try_value) => Ok(try_value),
        Err(error) => {
            let mut found_clause = None;

            for catch_clause in catch_clauses {
                let catch_value = interpreter.get_cadr(catch_clause)
                    .map_err(|_| interpreter.make_invalid_argument_error(
                        "The catch clauses of special form `try' must have two items at least."
                    ))?;

                let catch_symbol = match catch_value {
                    Value::Symbol(symbol) => symbol,
                    _ => return interpreter.make_invalid_argument_error(
                        "The first item of catch clause of the special form `try' must be a symbol."
                    ).into_result(),
                };

                if catch_symbol == error.get_symbol() {
                    found_clause = Some(catch_clause);
                    break;
                }
            }

            match found_clause {
                Some(catch_clause) => {
                    let catch_code = interpreter.get_cddr(catch_clause)
                        .map_err(|_| interpreter.make_invalid_argument_error(
                            "The catch clauses of special form `try' must have two items at least."
                        ))?;

                    match catch_code {
                        Value::Symbol(symbol_id) => {
                            let symbol = interpreter.get_symbol(symbol_id)?;

                            if symbol.is_nil() {
                                Ok(interpreter.intern_nil_symbol_value())
                            } else {
                                return interpreter.make_generic_execution_error(
                                    ""
                                ).into_result()
                            }
                        },
                        Value::Cons(cons_id) => {
                            let values = interpreter.cons_to_vec(cons_id)
                                .map_err(|err| interpreter.make_generic_execution_error_caused(
                                    "",
                                    err
                                ))?;

                            _lib::execute_forms(
                                interpreter,
                                environment_id,
                                values
                            )
                        },
                        _ => unreachable!()
                    }
                },
                None => Err(error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    // todo: ensure this test is fine
    #[test]
    fn returns_result_of_try_clause_if_it_was_ok() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            Value::Integer(1),
            interpreter.execute("(try (progn 1) (catch cute-error))").unwrap()
        );
        assert_eq!(
            Value::Integer(2),
            interpreter.execute("(try (progn 1 2) (catch cute-error))").unwrap()
        );
    }

    // todo: ensure this test is fine
    #[test]
    fn able_to_catch_error() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            Value::Integer(1),
            interpreter.execute("(try (progn 1 (throw cute-error)) (catch cute-error 1))").unwrap()
        );

        assert_eq!(
            Value::Integer(2),
            interpreter.execute("(try (progn 1 (throw cute-error)) (catch cute-error 1 2))").unwrap()
        );
    }

    // todo: ensure this test is fine
    #[test]
    fn if_error_cannot_be_catch_then_it_returns_it() {
        let mut interpreter = Interpreter::new();

        let symbol_id = interpreter.execute("(try (progn 1 (throw not-a-cute-error)) (catch cute-error 1))")
            .err()
            .unwrap()
            .get_symbol();

        assert_eq!(
            "not-a-cute-error",
            interpreter.get_symbol_name(symbol_id).unwrap()
        );
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_error_when_catch_clause_thrown_an_error() {
        let mut interpreter = Interpreter::new();

        let symbol_id = interpreter.execute("(try (progn 1 (throw cute-error)) (catch cute-error (throw not-a-cute-error)))")
            .err()
            .unwrap()
            .get_symbol();

        assert_eq!(
            "not-a-cute-error",
            interpreter.get_symbol_name(symbol_id).unwrap()
        );
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_err_when_not_enough_arguments_was_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(try 1)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_err_when_catch_clause_has_invalid_count_of_items() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(try 1 ())");
        assertion::assert_invalid_argument_error(&result);

        let result = interpreter.execute("(try 1 (catch))");
        assertion::assert_invalid_argument_error(&result);
    }
}
