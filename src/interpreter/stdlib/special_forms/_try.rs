use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::cons::cons_arena::ConsId;

fn parse_catch_clauses(interpreter: &mut Interpreter, clauses: Vec<Value>) -> Result<Vec<ConsId>, Error> {
    let mut catch_clauses = Vec::new();

    for clause in clauses {
        match clause {
            Value::Cons(cons_id) => {
                match interpreter.get_car(cons_id) {
                    Ok(Value::Symbol(symbol)) if symbol.get_name() == "catch" => {
                        catch_clauses.push(cons_id)
                    },
                    Ok(_) => return interpreter.make_invalid_argument_error(
                        "The first item of catch clauses must be a catch symbol."
                    ),
                    Err(error) => return interpreter.make_generic_execution_error_caused(
                        "",
                        error
                    )
                }
            }
            _ => return interpreter.make_invalid_argument_error(
                "The clauses of special form `try' must be lists."
            )
        }
    }

    for clause in &catch_clauses {
        match interpreter.get_cddr(*clause) {
            Ok(_) => {},
            Err(_) => return interpreter.make_invalid_argument_error(
                "The clauses of special form `try' must be lists with two items at least."
            )
        }
    }

    Ok(catch_clauses)
}

pub fn _try(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() < 2 {
        return interpreter.make_invalid_argument_count_error(
            "Special form `try' must take at least two arguments"
        );
    }

    let mut values = values;

    let try_code = values.remove(0);
    let clauses = values;

    let catch_clauses = match parse_catch_clauses(interpreter, clauses) {
        Ok(clauses) => clauses,
        Err(error) => return Err(error)
    };

    let try_result = interpreter.execute_value(environment, try_code);

    match try_result {
        Ok(try_value) => Ok(try_value),
        Err(error) => {
            let mut found_clause = None;

            for catch_clause in catch_clauses {
                let catch_symbol = match interpreter.get_cadr(catch_clause) {
                    Ok(Value::Symbol(symbol)) => symbol,
                    Ok(_) => return interpreter.make_invalid_argument_error(
                        "The first item of catch clause of the special form `try' must be a symbol."
                    ),
                    _  => return interpreter.make_invalid_argument_error(
                        "The catch clauses of special form `try' must have two items at least."
                    )
                };

                if catch_symbol == error.get_symbol() {
                    found_clause = Some(catch_clause);
                    break;
                }
            }

            match found_clause {
                Some(catch_clause) => {
                    let catch_code = match interpreter.get_cddr(catch_clause) {
                        Ok(value) => value,
                        Err(_) => return interpreter.make_invalid_argument_error(
                            "The catch clauses of special form `try' must have two items at least."
                        )
                    };

                    match catch_code {
                        Value::Symbol(symbol) if symbol.is_nil() => Ok(interpreter.intern_nil()),
                        Value::Cons(cons_id) => {
                            let values = interpreter.cons_to_vec(cons_id);

                            let values = match values {
                                Ok(values) => values,
                                Err(error) => return interpreter.make_generic_execution_error_caused(
                                    "",
                                    error
                                )
                            };

                            super::_lib::execute_forms(
                                interpreter,
                                environment,
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

    #[test]
    fn if_error_cannot_be_catch_then_it_returns_it() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            "not-a-cute-error",
            interpreter.execute("(try (progn 1 (throw not-a-cute-error)) (catch cute-error 1))")
                .err()
                .unwrap()
                .get_symbol()
                .get_name()
        );
    }

    #[test]
    fn returns_error_when_catch_clause_thrown_an_error() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            "not-a-cute-error",
            interpreter.execute("(try (progn 1 (throw cute-error)) (catch cute-error (throw not-a-cute-error)))")
                .err()
                .unwrap()
                .get_symbol()
                .get_name()
        );
    }

    #[test]
    fn returns_err_when_not_enough_arguments_was_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(try 1)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_err_when_catch_clause_has_invalid_count_of_items() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(try 1 ())");
        assertion::assert_invalid_argument_error(&result);

        let result = interpreter.execute("(try 1 (catch))");
        assertion::assert_invalid_argument_error(&result);
    }
}
