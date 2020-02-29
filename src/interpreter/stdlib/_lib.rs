use crate::interpreter::symbol::SymbolId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::cons::cons_arena::ConsId;
use crate::interpreter::function::arguments::Arguments;
use crate::interpreter::function::Function;

enum ArgumentParsingMode {
    Ordinary,
    Optional,
    Rest,
    Keys,
}

pub fn value_to_string(interpreter: &Interpreter, value: Value) -> Result<String, Error> {
    match value {
        Value::Integer(int) => Ok(int.to_string()),
        Value::Float(float) => Ok(float.to_string()),
        Value::Boolean(boolean) => if boolean {
            Ok(String::from("#t"))
        } else {
            Ok(String::from("#f"))
        },
        Value::String(string_id) => {
            let string = interpreter.get_string(string_id)?;

            Ok(String::from(string.get_string()))
        },
        Value::Symbol(symbol_id) => {
            let string = interpreter.get_symbol_name(symbol_id)?;

            Ok(String::from(string))
        },
        Value::Keyword(keyword_id) => {
            let keyword = interpreter.get_keyword(keyword_id)?;

            let mut string = String::from(":");
            string.push_str(keyword.get_name());

            Ok(string)
        },
        Value::Cons(cons_id) => {
            let values = interpreter.list_to_vec(cons_id)?;

            let mut result = String::new();
            result.push_str("(");

            for value in values {
                result.push_str(&value_to_string(interpreter, value)?);
                result.push_str(" ");
            }

            result.remove(result.len() - 1);

            result.push_str(")");
            Ok(result)
        },
        Value::Object(object_id) => {
            let items = interpreter.get_items(object_id)?;

            let mut result = String::new();
            result.push_str("{");

            for (symbol_id, value) in items {
                let mut name = String::from(":");
                name.push_str(interpreter.get_symbol_name(*symbol_id)?);
                let string = value_to_string(interpreter, *value)?;

                result.push_str(&name);
                result.push_str(" ");
                result.push_str(&string);
                result.push_str(" ");
            }

            result.remove(result.len() - 1);
            result.push_str("}");

            Ok(result)
        },
        Value::Function(function_id) => {
            let function = interpreter.get_function(function_id)?;

            let string = match function {
                Function::Interpreted(_) => String::from("<function>"),
                Function::Builtin(_) => String::from("<builtin-function>"),
                Function::Macro(_) => String::from("<macro>"),
                Function::SpecialForm(_) => String::from("<special-form>"),
            };

            Ok(string)
        }
    }
}

pub fn read_as_string(interpreter: &Interpreter, value: Value) -> Result<&String, Error> {
    let string_id = match value {
        Value::String(string_id) => string_id,
        _ => return interpreter.make_invalid_argument_error(
            "Expected string."
        ).into_result()
    };

    let string = interpreter.get_string(string_id)
        .map_err(|err| interpreter.make_generic_execution_error_caused(
            "",
            err
        ))?;

    Ok(string.get_string())
}

pub fn read_as_int(interpreter: &Interpreter, value: Value) -> Result<i64, Error> {
    match value {
        Value::Integer(int) => Ok(int),
        _ => interpreter.make_invalid_argument_error(
        "Expected int."
        ).into_result()
    }
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
        Value::Cons(cons_id) => interpreter.list_to_vec(cons_id)
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
            interpreter.list_to_vec(cons_id)
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


#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod value_to_string {
        use super::*;

        #[test]
        fn returns_string_representation_of_values() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("1",                       "1"),
                ("1.1",                     "1.1"),
                ("#t",                      "#t"),
                ("#f",                      "#f"),
                (r#""string""#,             "string"),
                ("'symbol",                 "symbol"),
                (":keyword",                ":keyword"),
                ("'(a b c)",                "(a b c)"),
                ("{:key 'value}",           "{:key value}"),
                ("#(+ %1 %2)",              "<function>"),
                ("(flookup 'flookup)",      "<builtin-function>"),
                ("(function (macro () 1))", "<macro>"),
                ("(flookup 'cond)",         "<special-form>"),
            );

            for (code, expected) in pairs {
                let value = interpreter.execute(code).unwrap();
                let result = value_to_string(&mut interpreter, value).unwrap();

                assert_eq!(expected, result);
            }
        }
    }

    #[cfg(test)]
    mod read_as_string {
        use super::*;
        use crate::interpreter::lib::assertion;

        #[test]
        fn returns_correct_string() {
            let mut interpreter = Interpreter::new();

            let value = interpreter.make_string_value(String::from("test"));
            let result = read_as_string(
                &mut interpreter,
                value
            );

            assert_eq!("test", result.unwrap());
        }

        #[test]
        fn returns_invalid_argument_when_not_a_string_value_were_passed() {
            let mut interpreter = Interpreter::new();

            let not_string_values = vec!(
                Value::Integer(1),
                Value::Float(1.1),
                Value::Boolean(true),
                Value::Boolean(false),
                interpreter.intern_symbol_value("test"),
                interpreter.make_keyword_value(String::from("test")),
                interpreter.make_cons_value(Value::Integer(1), Value::Integer(2)),
                interpreter.make_object_value(),
                interpreter.execute("#(+ %1 %2)").unwrap()
            );

            for not_string_value in not_string_values {
                let result = read_as_string(
                    &mut interpreter,
                    not_string_value
                );
                assertion::assert_invalid_argument_error(&result);
            }
        }
    }

    // todo: add tests here
    #[cfg(test)]
    mod parse_arguments {
        use super::*;
    }

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

}
