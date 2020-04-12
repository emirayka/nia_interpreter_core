use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::cons::ConsId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::function::Arguments;

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

pub fn parse_arguments_from_value(
    interpreter: &mut Interpreter,
    value: Value
) -> Result<Arguments, Error> {
    let arguments = match value {
        Value::Cons(cons_id) => interpreter.list_to_vec(cons_id)
            .map_err(|_| interpreter.make_generic_execution_error(""))?,
        Value::Symbol(symbol_id) => {
            if interpreter.symbol_is_nil(symbol_id)? {
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

// todo: add tests here
#[cfg(test)]
mod tests {
    use super::*;
}
