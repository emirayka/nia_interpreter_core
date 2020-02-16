use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::environment_arena::EnvironmentId;

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
        None => Ok(interpreter.intern_nil())
    }
}

pub fn read_let_definitions(interpreter: &mut Interpreter, value: Value) -> Result<Vec<Value>, Error> {
    let definitions = match value {
        Value::Cons(cons_id) => match interpreter.cons_to_vec(cons_id) {
            Ok(vec) => vec,
            _ => return interpreter.make_empty_error()
        },
        Value::Symbol(symbol) if symbol.is_nil() => Vec::new(),
        _ => return interpreter.make_invalid_argument_error("")
    };

    for definition in &definitions {
        match definition {
            Value::Cons(_) => {},
            Value::Symbol(symbol) if symbol.is_nil() => return interpreter.make_invalid_argument_error(""),
            Value::Symbol(_) => {},
            _ => return interpreter.make_invalid_argument_error("")
        }
    };

    Ok(definitions)
}

pub fn convert_vector_of_values_to_vector_of_symbol_names(values: Vec<Value>) -> Result<Vec<String>, ()> {
    let mut result = Vec::new();

    for value in values {
        let name = match value {
            Value::Symbol(symbol) => String::from(symbol.get_name()),
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
    mod execute_forms {
        use super::*;
        use crate::interpreter::lib::assertion;

        #[test]
        fn returns_the_result_of_execution_of_the_last_form() {
            let mut interpreter = Interpreter::new();
            let name = interpreter.intern_symbol("test");

            interpreter.define_variable(
                interpreter.get_root_environment(),
                &name,
                Value::Integer(10)
            ).unwrap();

            let forms = vec!(
                Value::Integer(1),
                Value::Symbol(name)
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
                interpreter.intern("test")
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
            let nil = interpreter.intern_nil();

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
            let mut result = read_let_definitions(
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
                interpreter.intern("a"),
                interpreter.intern("b"),
                interpreter.intern("c"),
            );

            let expected = vec!(
                String::from("a"),
                String::from("b"),
                String::from("c"),
            );

            let result = convert_vector_of_values_to_vector_of_symbol_names(
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
                    interpreter.intern("a"),
                    interpreter.intern("b"),
                    interpreter.intern("c"),
                    incorrect_item
                );

                let result = convert_vector_of_values_to_vector_of_symbol_names(
                    values
                );

                assertion::assert_error(&result);
            }
        }
    }
}

