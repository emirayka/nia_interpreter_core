use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::cons::Cons;
use crate::interpreter::error::Error;
use crate::interpreter::value::Value;

use crate::interpreter::stdlib::special_forms::_lib::infect_special_form;

fn parse_catch_clauses(interpreter: &mut Interpreter, clauses: Vec<Value>) -> Result<Vec<Cons>, Error> {
    let mut catch_clauses = Vec::new();

    for clause in clauses {
        match clause {
            Value::Cons(cons) => {
                match cons.get_car() {
                    Value::Symbol(symbol) if symbol.get_name() == "catch" => {
                        catch_clauses.push(cons)
                    },
                    _ => return Err(Error::invalid_argument(
                        interpreter,
                        "The first item of clauses must be lists."
                    ))
                }
            },
            _ => return Err(Error::invalid_argument(
                interpreter,
                "The clauses of special form `try' must be lists."
            ))
        }
    }

    for clause in &catch_clauses {
        match clause.get_cddr() {
            Ok(_) => {},
            Err(_) => return Err(Error::invalid_argument(
                interpreter,
                "The clauses of special form `try' must be lists with two items at least."
            ))
        }
    }

    Ok(catch_clauses)
}

fn _try(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() < 2 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Special form `try' must take at least two arguments"
        ));
    }

    let mut values = values;

    let try_code = values.remove(0);
    let clauses = values;

    let catch_clauses = match parse_catch_clauses(interpreter, clauses) {
        Ok(clauses) => clauses,
        Err(error) => return Err(error)
    };

    let try_result = interpreter.execute_value(environment, &try_code);

    match try_result {
        Ok(try_value) => Ok(try_value),
        Err(error) => {
            let mut found_clause = None;

            for catch_clause in catch_clauses {
                let catch_symbol = match catch_clause.get_cadr() {
                    Ok(Value::Symbol(symbol)) => symbol,
                    Ok(_) => return Err(Error::invalid_argument(
                        interpreter,
                        "The first item of catch clause of the special form `try' must be a symbol."
                    )),
                    _  => return Err(Error::invalid_argument(
                        interpreter,
                        "The catch clauses of special form `try' must have two items at least."
                    ))
                };

                if catch_symbol == &error.get_symbol() {
                    found_clause = Some(catch_clause);
                    break;
                }
            }

            match found_clause {
                Some(catch_clause) => {
                    let catch_code = match catch_clause.get_cddr() {
                        Ok(value) => value,
                        Err(_) => return Err(Error::invalid_argument(
                            interpreter,
                            "The catch clauses of special form `try' must have two items at least."
                        ))
                    };

                    match catch_code {
                        Value::Symbol(symbol) if symbol.is_nil() => Ok(interpreter.intern_nil()),
                        Value::Cons(cons) => super::_lib::execute_forms(
                            interpreter,
                            environment,
                            cons.to_vec()
                        ),
                        _ => unreachable!()
                    }
                },
                None => Err(error)
            }
        }
    }
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_special_form(interpreter, "try", _try)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::error::assertion;

    #[test]
    fn returns_result_of_try_clause_if_it_was_ok() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();
        super::super::throw::infect(&mut interpreter).unwrap();
        super::super::progn::infect(&mut interpreter).unwrap();

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
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();
        super::super::throw::infect(&mut interpreter).unwrap();
        super::super::progn::infect(&mut interpreter).unwrap();

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
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();
        super::super::throw::infect(&mut interpreter).unwrap();
        super::super::progn::infect(&mut interpreter).unwrap();

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
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();
        super::super::throw::infect(&mut interpreter).unwrap();
        super::super::progn::infect(&mut interpreter).unwrap();

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
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();
        super::super::throw::infect(&mut interpreter).unwrap();
        super::super::progn::infect(&mut interpreter).unwrap();

        let result = interpreter.execute("(try 1)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_err_when_catch_clause_has_invalid_count_of_items() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();
        super::super::throw::infect(&mut interpreter).unwrap();
        super::super::progn::infect(&mut interpreter).unwrap();

        let result = interpreter.execute("(try 1 ())");
        assertion::assert_invalid_argument_error(&result);

        let result = interpreter.execute("(try 1 (catch))");
        assertion::assert_invalid_argument_error(&result);
    }
}
