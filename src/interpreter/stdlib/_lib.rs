use crate::interpreter::symbol::SymbolId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::environment_arena::EnvironmentId;

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

        #[test]
        fn returns_ok_on_ordinary_symbols() {
            let mut interpreter = Interpreter::new();
            let symbol_id = interpreter.intern("test");

            let result = check_if_symbol_assignable(&mut interpreter, symbol_id);

            assert!(result.is_ok());
        }

        #[test]
        fn returns_error_on_constants() {
            for_constants(|interpreter, string| {
                let symbol_id = interpreter.intern(&string);

                let result = check_if_symbol_assignable(interpreter, symbol_id);

                assertion::assert_invalid_argument_error(&result);
            })
        }

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

