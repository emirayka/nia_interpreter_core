use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::ConsId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::value::FunctionArguments;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            if interpreter.symbol_is_not_nil(symbol_id)? {
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
                if interpreter.symbol_is_not_nil(symbol_id)? {
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
                return Error::invalid_argument_error("")
                    .into_result()
            }

            let symbol_name = interpreter.get_symbol_name(symbol_id)?;

            Ok(String::from(symbol_name))
        },
        _ => return Error::invalid_argument_error("")
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

fn parse_arguments(interpreter: &mut Interpreter, values: Vec<Value>) -> Result<FunctionArguments, Error> {
    let mut arguments = FunctionArguments::new();
    let mut mode = ArgumentParsingMode::Ordinary;

    for value in values {
        match value {
            Value::Symbol(symbol_id) => {
                let is_constant = interpreter.check_if_symbol_constant(symbol_id)
                    .map_err(|err| Error::generic_execution_error_caused(
                        "",
                        err
                    ))?;

                let symbol = match interpreter.get_symbol(symbol_id) {
                    Ok(symbol) => symbol,
                    Err(error) => return Error::generic_execution_error_caused(
                        "",
                        error
                    ).into_result()
                };

                if is_constant {
                    return Error::invalid_argument_error(
                        "Cannot set constants as arguments"
                    ).into_result()
                }

                let symbol_name = symbol.get_name();

                if symbol_name == "#opt" {
                    if mode == ArgumentParsingMode::Ordinary {
                        mode = ArgumentParsingMode::Optional;
                    } else {
                        return Error::generic_execution_error(
                            "Invalid argument specification: optional arguments may occur only after ordinary arguments."
                        ).into_result();
                    }
                    continue;
                } else if symbol_name == "#rest" {
                    if mode == ArgumentParsingMode::Ordinary || mode == ArgumentParsingMode::Optional {
                        mode = ArgumentParsingMode::Rest;
                    } else {
                        return Error::generic_execution_error(
                            "Invalid argument specification: rest argument may occur only after ordinary or optional arguments."
                        ).into_result();
                    }
                    continue;
                } else if symbol_name == "#keys" {
                    if mode == ArgumentParsingMode::Ordinary {
                        mode = ArgumentParsingMode::Keys;
                    } else {
                        return Error::generic_execution_error(
                            "Invalid argument specification: key arguments may occur only after ordinary arguments."
                        ).into_result();
                    }
                    continue;
                }

                match mode {
                    ArgumentParsingMode::Ordinary => {
                        arguments.add_ordinary_argument(symbol.get_name().clone())
                            .map_err(|_| Error::generic_execution_error(""))?;
                    },
                    ArgumentParsingMode::Optional => {
                        arguments.add_optional_argument(
                            symbol.get_name().clone(),
                            None,
                            None
                        ).map_err(|_| Error::generic_execution_error(""))?;
                    },
                    ArgumentParsingMode::Rest => {
                        arguments.add_rest_argument(symbol.get_name().clone())
                            .map_err(|_| Error::generic_execution_error(""))?;
                    },
                    ArgumentParsingMode::Keys => {
                        arguments.add_key_argument(
                            symbol.get_name().clone(),
                            None,
                            None
                        ).map_err(|_| Error::generic_execution_error(""))?;
                    },
                }

            },
            Value::Cons(cons_id) => {
                match mode {
                    ArgumentParsingMode::Ordinary => {
                        return Error::invalid_argument_error("")
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
                        ).map_err(|_| Error::generic_execution_error(""))?;
                    },
                    ArgumentParsingMode::Rest => {
                        return Error::invalid_argument_error("")
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
                        ).map_err(|_| Error::generic_execution_error(""))?;
                    },
                }
            },
            _ => return Error::invalid_argument_error("")
                .into_result()
        }
    }

    Ok(arguments)
}

pub fn read_as_arguments(
    interpreter: &mut Interpreter,
    value: Value
) -> Result<FunctionArguments, Error> {
    let arguments = match value {
        Value::Cons(cons_id) => interpreter.list_to_vec(cons_id)
            .map_err(|_| Error::generic_execution_error(""))?,
        Value::Symbol(symbol_id) => {
            if interpreter.symbol_is_nil(symbol_id)? {
                Vec::new()
            } else {
                return Error::invalid_argument_error("")
                    .into_result()
            }
        },
        _ => {
            return Error::invalid_argument_error("")
                .into_result()
        }
    };

    parse_arguments(interpreter, arguments)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn assert_returns_correctly_parsed_arguments() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            // tests for each variation of every argument type
            (
                "'()",
                {
                    FunctionArguments::new()
                }
            ),
            (
                "'(a)",
                {
                    let mut arguments = FunctionArguments::new();

                    arguments.add_ordinary_argument(String::from("a")).unwrap();

                    arguments
                }
            ),
            (
                "'(#opt a)",
                {
                    let mut arguments = FunctionArguments::new();

                    arguments.add_optional_argument(String::from("a"), None, None).unwrap();

                    arguments
                }
            ),
            (
                "'(#opt (a 1))",
                {
                    let mut arguments = FunctionArguments::new();

                    arguments.add_optional_argument(String::from("a"), Some(Value::Integer(1)), None).unwrap();

                    arguments
                }
            ),
            (
                "'(#opt (a 1 a?))",
                {
                    let mut arguments = FunctionArguments::new();

                    arguments.add_optional_argument(String::from("a"), Some(Value::Integer(1)), Some(String::from("a?"))).unwrap();

                    arguments
                }
            ),
            (
                "'(#rest a)",
                {
                    let mut arguments = FunctionArguments::new();

                    arguments.add_rest_argument(String::from("a")).unwrap();

                    arguments
                }
            ),
            (
                "'(#keys a)",
                {
                    let mut arguments = FunctionArguments::new();

                    arguments.add_key_argument(String::from("a"), None, None).unwrap();

                    arguments
                }
            ),
            (
                "'(#keys (a 1))",
                {
                    let mut arguments = FunctionArguments::new();

                    arguments.add_key_argument(String::from("a"), Some(Value::Integer(1)), None).unwrap();

                    arguments
                }
            ),
            (
                "'(#keys (a 1 a?))",
                {
                    let mut arguments = FunctionArguments::new();

                    arguments.add_key_argument(String::from("a"), Some(Value::Integer(1)), Some(String::from("a?"))).unwrap();

                    arguments
                }
            ),
            // tests for combinations
            (
                "'(a b)",
                {
                    let mut arguments = FunctionArguments::new();

                    arguments.add_ordinary_argument(String::from("a")).unwrap();
                    arguments.add_ordinary_argument(String::from("b")).unwrap();

                    arguments
                }
            ),
            (
                "'(a #opt b)",
                {
                    let mut arguments = FunctionArguments::new();

                    arguments.add_ordinary_argument(String::from("a")).unwrap();
                    arguments.add_optional_argument(String::from("b"), None, None).unwrap();

                    arguments
                }
            ),
            (
                "'(a #rest b)",
                {
                    let mut arguments = FunctionArguments::new();

                    arguments.add_ordinary_argument(String::from("a")).unwrap();
                    arguments.add_rest_argument(String::from("b")).unwrap();

                    arguments
                }
            ),
            (
                "'(a #opt b c #rest d)",
                {
                    let mut arguments = FunctionArguments::new();

                    arguments.add_ordinary_argument(String::from("a")).unwrap();
                    arguments.add_optional_argument(String::from("b"), None, None).unwrap();
                    arguments.add_optional_argument(String::from("c"), None, None).unwrap();
                    arguments.add_rest_argument(String::from("d")).unwrap();

                    arguments
                }
            ),
            (
                "'(a #keys b)",
                {
                    let mut arguments = FunctionArguments::new();

                    arguments.add_ordinary_argument(String::from("a")).unwrap();
                    arguments.add_key_argument(String::from("b"), None, None).unwrap();

                    arguments
                }
            ),
        );

        for spec in specs {
            let expected = spec.1;

            let value = interpreter.execute(spec.0).unwrap();
            let result = read_as_arguments(
                &mut interpreter,
                value
            ).unwrap();

            assert_eq!(expected, result)
        }
    }

    #[test]
    fn assert_returns_error_when_invalid_argument_sets_were_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "'(#opt a #opt b)",
            "'(#opt a #keys b)",

            "'(#rest a b)",
            "'(#rest a #opt b)",
            "'(#rest a #rest b)",
            "'(#rest a #keys b)",

            "'(#keys a #opt b)",
            "'(#keys a #rest b)",
            "'(#keys a #keys b)",
        );

        for spec in specs {
            println!("{}", spec);
            let value = interpreter.execute(spec).unwrap();
            let result = read_as_arguments(
                &mut interpreter,
                value
            );

            assertion::assert_is_err(result);
        }
    }
}
