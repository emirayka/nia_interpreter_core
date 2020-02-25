use crate::interpreter::symbol::SymbolId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::cons::cons_arena::ConsId;
use crate::interpreter::function::arguments::Arguments;

enum ArgumentParsingMode {
    Ordinary,
    Optional,
    Rest,
    Keys,
}

fn extract_three_items_from_cons(
    interpreter: &mut Interpreter,
    cons_id: ConsId
) -> Result<(Value, Option<Value>, Option<Value>), Error> {
    let first = interpreter.get_car(cons_id)?;
    let mut second = None;
    let mut third = None;

    let cdr = interpreter.get_cdr(cons_id)?;

    let cons_id = match cdr {
        Value::Cons(cons_id) => {
            second = Some(interpreter.get_car(cons_id)?);

            Some(cons_id)
        },
        Value::Symbol(symbol_id) => {
            let symbol = interpreter.get_symbol(symbol_id)?;

            if !symbol.is_nil() {
                second = Some(Value::Symbol(symbol_id));
            }

            None
        },
        value @ _ => {
            second = Some(value);

            None
        }
    };

    if let Some(cons_id) = cons_id {
        let cdr = interpreter.get_cdr(cons_id)?;

        match cdr {
            Value::Cons(cons_id) => {
                third = Some(interpreter.get_car(cons_id)?);
            },
            Value::Symbol(symbol_id) => {
                let symbol = interpreter.get_symbol(symbol_id)?;

                if !symbol.is_nil() {
                    third = Some(Value::Symbol(symbol_id));
                }
            },
            value @ _ => {
                third = Some(value);
            }
        }
    }

    Ok((first, second, third))
}

fn extract_argument_name(interpreter: &mut Interpreter, value: Value) -> Result<String, Error> {
    match value {
        Value::Symbol(symbol_id) => {
            if !interpreter.check_if_symbol_assignable(symbol_id)? {
                return interpreter.make_invalid_argument_error("")
                    .into_result()
            }

            let symbol_name = interpreter.get_symbol_name(symbol_id)?;

            Ok(String::from(symbol_name))
        },
        _ => return interpreter.make_invalid_argument_error("")
            .into_result()
    }
}

fn extract_optional_argument_from_cons(
    interpreter: &mut Interpreter,
    cons_id: ConsId
) -> Result<(String, Option<Value>, Option<String>), Error> {
    let triplet = extract_three_items_from_cons(interpreter, cons_id)?;

    let first = extract_argument_name(interpreter, triplet.0)?;

    let second = triplet.1;

    let third = match triplet.2 {
        Some(value) => {
            let argument_name = extract_argument_name(interpreter, value)?;

            Some(argument_name)
        },
        None => None
    };

    Ok((first, second, third))
}

fn parse_arguments(interpreter: &mut Interpreter, values: Vec<Value>) -> Result<Arguments, Error> {
    let mut arguments = Arguments::new();
    let mut mode = ArgumentParsingMode::Ordinary;

    for value in values {
        match value {
            Value::Symbol(symbol_id) => {
                let is_constant = interpreter.check_if_symbol_constant(symbol_id)
                    .map_err(|err| interpreter.make_generic_execution_error_caused(
                        "",
                        err
                    ))?;

                let symbol = match interpreter.get_symbol(symbol_id) {
                    Ok(symbol) => symbol,
                    Err(error) => return interpreter.make_generic_execution_error_caused(
                        "",
                        error
                    ).into_result()
                };

                if is_constant {
                    return interpreter.make_invalid_argument_error(
                        "Cannot set constants as arguments"
                    ).into_result()
                }

                let symbol_name = symbol.get_name();

                if symbol_name == "#opt" {
                    mode = ArgumentParsingMode::Optional;
                    continue;
                } else if symbol_name == "#rest" {
                    mode = ArgumentParsingMode::Rest;
                    continue;
                } else if symbol_name == "#keys" {
                    mode = ArgumentParsingMode::Keys;
                    continue;
                }

                match mode {
                    ArgumentParsingMode::Ordinary => {
                        arguments.add_ordinary_argument(symbol.get_name().clone())
                            .map_err(|_| interpreter.make_generic_execution_error(""))?;
                    },
                    ArgumentParsingMode::Optional => {
                        arguments.add_optional_argument(
                            symbol.get_name().clone(),
                            None,
                            None
                        ).map_err(|_| interpreter.make_generic_execution_error(""))?;
                    },
                    ArgumentParsingMode::Rest => {
                        arguments.add_rest_argument(symbol.get_name().clone())
                            .map_err(|_| interpreter.make_generic_execution_error(""))?;
                    },
                    ArgumentParsingMode::Keys => {
                        arguments.add_key_argument(
                            symbol.get_name().clone(),
                            None,
                            None
                        ).map_err(|_| interpreter.make_generic_execution_error(""))?;
                    },
                }

            },
            Value::Cons(cons_id) => {
                match mode {
                    ArgumentParsingMode::Ordinary => {
                        return interpreter.make_invalid_argument_error("")
                            .into_result();
                    },
                    ArgumentParsingMode::Optional => {
                        let triplet = extract_optional_argument_from_cons(
                            interpreter,
                            cons_id
                        )?;

                        arguments.add_optional_argument(
                            triplet.0,
                            triplet.1,
                            triplet.2
                        ).map_err(|_| interpreter.make_generic_execution_error(""))?;
                    },
                    ArgumentParsingMode::Rest => {
                        return interpreter.make_invalid_argument_error("")
                            .into_result();
                    },
                    ArgumentParsingMode::Keys => {
                        let triplet = extract_optional_argument_from_cons(
                            interpreter,
                            cons_id
                        )?;

                        arguments.add_key_argument(
                            triplet.0,
                            triplet.1,
                            triplet.2
                        ).map_err(|_| interpreter.make_generic_execution_error(""))?;
                    },
                }
            },
            _ => return interpreter.make_invalid_argument_error("")
                .into_result()
        }
    }

    Ok(arguments)
}

pub fn parse_arguments_from_value(interpreter: &mut Interpreter, value: Value) -> Result<Arguments, Error> {
    let arguments = match value {
        Value::Cons(cons_id) => interpreter.cons_to_vec(cons_id)
            .map_err(|_| interpreter.make_generic_execution_error(""))?,
        Value::Symbol(symbol_id) => {
            let symbol = interpreter.get_symbol(symbol_id)?;

            if symbol.is_nil() {
                Vec::new()
            } else {
                return interpreter.make_invalid_argument_error("")
                    .into_result()
            }
        },
        _ => {
            return interpreter.make_invalid_argument_error("")
                .into_result()
        }
    };

    parse_arguments(interpreter, arguments)
}

pub fn check_if_symbol_assignable(interpreter: &mut Interpreter, symbol_id: SymbolId) -> Result<(), Error> {
    match interpreter.check_if_symbol_assignable(symbol_id) {
        Ok(true) => {},
        Ok(false) => return interpreter.make_invalid_argument_error("").into_result(),
        Err(error) => return interpreter.make_generic_execution_error_caused(
            "",
            error
        ).into_result()
    }

    Ok(())
}

pub fn execute_forms(
    interpreter: &mut Interpreter,
    execution_environment: EnvironmentId,
    forms: Vec<Value>
) -> Result<Value, Error> {
    let mut last_result = None;

    for form in forms {
        let result = interpreter.execute_value(execution_environment, form)?;
        last_result = Some(result);
    }

    match last_result {
        Some(value) => Ok(value),
        None => Ok(interpreter.intern_nil_symbol_value())
    }
}

pub fn read_let_definitions(interpreter: &mut Interpreter, value: Value) -> Result<Vec<Value>, Error> {
    let definitions = match value {
        Value::Cons(cons_id) => {
            interpreter.cons_to_vec(cons_id)
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

pub fn convert_vector_of_values_to_vector_of_symbol_names(
    interpreter: &mut Interpreter,
    values: Vec<Value>
) -> Result<Vec<String>, ()> {
    let mut result = Vec::new();

    for value in values {
        let name = match value {
            Value::Symbol(symbol_id) => {
                let symbol = interpreter.get_symbol(symbol_id);

                let symbol = match symbol {
                    Ok(symbol) => symbol,
                    Err(_) => return Err(())
                };

                String::from(symbol.get_name())
            },
            _ => return Err(())
        };

        result.push(name);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod check_if_symbol_is_assignable {
        use super::*;
        use crate::interpreter::lib::testing_helpers::{for_constants, for_special_symbols};
        use crate::interpreter::lib::assertion;

        // todo: ensure this test is fine
    #[test]
        fn returns_ok_on_ordinary_symbols() {
            let mut interpreter = Interpreter::new();
            let symbol_id = interpreter.intern("test");

            let result = check_if_symbol_assignable(&mut interpreter, symbol_id);

            assert!(result.is_ok());
        }

        // todo: ensure this test is fine
    #[test]
        fn returns_error_on_constants() {
            for_constants(|interpreter, string| {
                let symbol_id = interpreter.intern(&string);

                let result = check_if_symbol_assignable(interpreter, symbol_id);

                assertion::assert_invalid_argument_error(&result);
            })
        }

        // todo: ensure this test is fine
    #[test]
        fn returns_error_on_special_symbols() {
            for_special_symbols(|interpreter, string| {
                let symbol_id = interpreter.intern(&string);

                let result = check_if_symbol_assignable(interpreter, symbol_id);

                assertion::assert_invalid_argument_error(&result);
            })
        }
    }

    #[cfg(test)]
    mod execute_forms {
        use super::*;
        use crate::interpreter::lib::assertion;

        // todo: ensure this test is fine
    #[test]
        fn returns_the_result_of_execution_of_the_last_form() {
            let mut interpreter = Interpreter::new();
            let symbol_id = interpreter.intern("test");

            interpreter.define_variable(
                interpreter.get_root_environment(),
                symbol_id,
                Value::Integer(10)
            ).unwrap();

            let forms = vec!(
                Value::Integer(1),
                Value::Symbol(symbol_id)
            );

            let root_env = interpreter.get_root_environment();

            let result = execute_forms(
                &mut interpreter,
                root_env,
                forms
            );

            assert_eq!(Value::Integer(10), result.unwrap());
        }

        // todo: ensure this test is fine
    #[test]
        fn returns_err_when_execution_failed() {
            let mut interpreter = Interpreter::new();

            let forms = vec!(
                Value::Integer(1),
                interpreter.intern_symbol_value("test")
            );

            let root_env = interpreter.get_root_environment();

            let result = execute_forms(
                &mut interpreter,
                root_env,
                forms
            );

            assertion::assert_error(&result);
        }
    }

    #[cfg(test)]
    mod read_let_definitions {
        use super::*;
        use crate::interpreter::lib::assertion;

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

            assertion::assert_error(&result);
        }
    }

    #[cfg(test)]
    mod convert_vector_of_values_to_vector_of_symbol_names {
        use super::*;
        use crate::interpreter::lib::assertion;

        // todo: ensure this test is fine
    #[test]
        fn returns_vector_of_symbol_names() {
            let mut interpreter = Interpreter::new();
            let values = vec!(
                interpreter.intern_symbol_value("a"),
                interpreter.intern_symbol_value("b"),
                interpreter.intern_symbol_value("c"),
            );

            let expected = vec!(
                String::from("a"),
                String::from("b"),
                String::from("c"),
            );

            let result = convert_vector_of_values_to_vector_of_symbol_names(
                &mut interpreter,
                values
            );

            assert_eq!(expected, result.unwrap());
        }

        // todo: ensure this test is fine
    #[test]
        fn returns_err_when_not_a_symbol_were_provided() {
            let mut interpreter = Interpreter::new();

            let incorrect_items = vec!(
                Value::Integer(1),
                Value::Float(1.1),
                Value::Boolean(true),
                Value::Boolean(false),
                interpreter.intern_string_value(String::from("string")),
                interpreter.intern_keyword_value(String::from("keyword")),
            );

            for incorrect_item in incorrect_items {
                let values = vec!(
                    interpreter.intern_symbol_value("a"),
                    interpreter.intern_symbol_value("b"),
                    interpreter.intern_symbol_value("c"),
                    incorrect_item
                );

                let result = convert_vector_of_values_to_vector_of_symbol_names(
                    &mut interpreter,
                    values
                );

                assertion::assert_error(&result);
            }
        }
    }
}

