use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::ConsId;
use crate::interpreter::value::Value;

fn read_catch_clauses(
    interpreter: &mut Interpreter,
    clauses: Vec<Value>,
) -> Result<Vec<ConsId>, Error> {
    let mut catch_clauses = Vec::new();

    for clause in clauses {
        match clause {
            Value::Cons(cons_id) => {
                let car = interpreter
                    .get_car(cons_id)
                    .map_err(|err| Error::generic_execution_error_caused("", err))?;

                match car {
                    Value::Symbol(symbol_id) => {
                        let symbol = interpreter.get_symbol(symbol_id)?;

                        if symbol.get_name() == "catch" {
                            catch_clauses.push(cons_id)
                        }
                    }
                    _ => {
                        return Error::invalid_argument_error(
                            "The first item of catch clauses must be a catch symbol.",
                        )
                        .into()
                    }
                }
            }
            _ => {
                return Error::invalid_argument_error(
                    "The clauses of special form `try' must be lists.",
                )
                .into()
            }
        }
    }

    for clause in &catch_clauses {
        interpreter.get_cddr(*clause).map_err(|_| {
            Error::invalid_argument_error(
                "The clauses of special form `try' must be lists with two items at least.",
            )
        })?;
    }

    Ok(catch_clauses)
}

pub fn _try(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() < 2 {
        return Error::invalid_argument_count_error(
            "Special form `try' must take at least two arguments",
        )
        .into();
    }

    let mut values = values;

    let try_code = values.remove(0);
    let clauses = values;

    let catch_clauses = read_catch_clauses(interpreter, clauses)?;
    let try_result = interpreter.execute_value(environment_id, try_code);

    match try_result {
        Ok(try_value) => Ok(try_value),
        Err(error) => {
            let mut found_clause = None;

            for catch_clause in catch_clauses {
                let catch_value = interpreter.get_cadr(catch_clause).map_err(|_| {
                    Error::invalid_argument_error(
                        "The catch clauses of special form `try' must have two items at least.",
                    )
                })?;

                let catch_value = interpreter.execute_value(environment_id, catch_value)?;

                let catch_symbol_id = match catch_value {
                    Value::Symbol(symbol) => symbol,
                    _ => return Error::invalid_argument_error(
                        "The first item of catch clause of the special form `try' must be a symbol."
                    ).into(),
                };

                let catch_symbol_name = interpreter
                    .get_symbol_name(catch_symbol_id)
                    .map_err(|err| Error::generic_execution_error_caused("", err))?;

                if catch_symbol_name == error.get_symbol_name() {
                    found_clause = Some(catch_clause);
                    break;
                }
            }

            match found_clause {
                Some(catch_clause) => {
                    let catch_code = interpreter.get_cddr(catch_clause).map_err(|_| {
                        Error::invalid_argument_error(
                            "The catch clauses of special form `try' must have two items at least.",
                        )
                    })?;

                    match catch_code {
                        Value::Symbol(symbol_id) => {
                            if interpreter.symbol_is_nil(symbol_id)? {
                                Ok(interpreter.intern_nil_symbol_value())
                            } else {
                                return Error::generic_execution_error("").into();
                            }
                        }
                        Value::Cons(cons_id) => {
                            let values = interpreter
                                .list_to_vec(cons_id)
                                .map_err(|err| Error::generic_execution_error_caused("", err))?;

                            library::execute_forms(interpreter, environment_id, &values)
                        }
                        _ => unreachable!(),
                    }
                }
                None => Err(error),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_result_of_try_clause_if_it_was_ok() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            ("(try (progn 1) (catch 'cute-error))", Value::Integer(1)),
            ("(try (progn 1 2) (catch 'cute-error))", Value::Integer(2)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, specs);
    }

    #[test]
    fn able_to_catch_error() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                "(try (progn 1 (throw 'cute-error)) (catch 'cute-error 1))",
                Value::Integer(1),
            ),
            (
                "(try (progn 1 (throw 'cute-error)) (catch 'cute-error 1 2))",
                Value::Integer(2),
            ),
        ];
    }

    #[test]
    fn if_error_cannot_be_catch_then_it_returns_it() {
        let mut interpreter = Interpreter::new();

        let error = interpreter
            .execute_in_main_environment(
                "(try (progn 1 (throw 'not-a-cute-error)) (catch 'cute-error 1))",
            )
            .err()
            .unwrap();

        nia_assert_equal("not-a-cute-error", error.get_symbol_name());
    }

    #[test]
    fn returns_error_when_catch_clause_thrown_an_error() {
        let mut interpreter = Interpreter::new();

        let error = interpreter
            .execute_in_main_environment(
                "(try (progn 1 (throw 'cute-error)) (catch 'cute-error (throw 'not-a-cute-error)))",
            )
            .err()
            .unwrap();

        nia_assert_equal("not-a-cute-error", error.get_symbol_name());
    }

    #[test]
    fn returns_err_when_not_enough_arguments_was_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute_in_main_environment("(try 1)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_err_when_catch_clause_has_invalid_count_of_items() {
        let mut interpreter = Interpreter::new();

        let specs = vec!["(try 1 ())", "(try 1 (catch))"];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, specs);
    }
}
