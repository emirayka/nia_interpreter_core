use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::library;

pub fn remove_last_item_from_root_list(
    interpreter: &mut Interpreter,
    name: &str,
) -> Result<(), Error> {
    let root_environment = interpreter.get_root_environment_id();
    let symbol_name = interpreter.intern_symbol_id(name);

    let root_variable = interpreter
        .lookup_variable(root_environment, symbol_name)?
        .ok_or_else(|| {
            Error::generic_execution_error("Cannot find variable.")
        })?;

    library::check_value_is_list(interpreter, root_variable)?;

    let mut items = library::read_as_vector(interpreter, root_variable)?;

    if items.len() == 0 {
        return Error::generic_execution_error(
            "Cannot remove item from empty list.",
        )
        .into();
    }

    items.remove(items.len() - 1);

    let list = interpreter.vec_to_list(items);

    interpreter.set_variable(root_environment, symbol_name, list)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;
    use std::panic::resume_unwind;

    const EMPTY_LIST_VARIABLE_SYMBOL_NAME: &'static str = "test-list-symbol";

    fn setup(items: Vec<Value>) -> Interpreter {
        let mut interpreter = Interpreter::new();

        let root_environment_id = interpreter.get_root_environment_id();
        let symbol =
            interpreter.intern_symbol_id(EMPTY_LIST_VARIABLE_SYMBOL_NAME);
        let list = interpreter.vec_to_list(items);

        interpreter
            .define_variable(root_environment_id, symbol, list)
            .unwrap();

        interpreter
    }

    fn assert_works_for_list(expected: Vec<Value>, vector: Vec<Value>) {
        let mut interpreter = setup(vector);

        remove_last_item_from_root_list(
            &mut interpreter,
            EMPTY_LIST_VARIABLE_SYMBOL_NAME,
        )
        .unwrap();

        let result = library::get_root_variable(
            &mut interpreter,
            EMPTY_LIST_VARIABLE_SYMBOL_NAME,
        )
        .unwrap();

        let result = library::read_as_vector(&mut interpreter, result).unwrap();

        utils::assert_vectors_deep_equal(&mut interpreter, expected, result)
    }

    #[test]
    fn removes_last_item_from_list() {
        assert_works_for_list(vec![], vec![Value::Integer(1)]);

        assert_works_for_list(
            vec![Value::Integer(1)],
            vec![Value::Integer(1), Value::Integer(2)],
        );
        assert_works_for_list(
            vec![Value::Integer(1), Value::Integer(2)],
            vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)],
        );
    }

    #[test]
    fn returns_generic_execution_error_if_list_is_empty() {
        let mut interpreter = setup(vec![]);

        let result = remove_last_item_from_root_list(
            &mut interpreter,
            EMPTY_LIST_VARIABLE_SYMBOL_NAME,
        );

        nia_assert_is_err(&result);
    }

    #[test]
    fn returns_invalid_argument_error_if_value_is_not_a_list() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            Value::Integer(1),
            Value::Float(1.1),
            Value::Boolean(false),
            Value::Boolean(true),
            interpreter.intern_string_value("string"),
            interpreter.intern_keyword_value("string"),
            interpreter.intern_symbol_value("symbol"),
            interpreter.make_object_value(),
        ];

        let root_environment_id = interpreter.get_root_environment_id();
        let symbol =
            interpreter.intern_symbol_id(EMPTY_LIST_VARIABLE_SYMBOL_NAME);

        interpreter
            .define_variable(root_environment_id, symbol, Value::Integer(0))
            .unwrap();

        for spec in specs {
            interpreter
                .set_variable(root_environment_id, symbol, spec)
                .unwrap();

            let result = remove_last_item_from_root_list(
                &mut interpreter,
                EMPTY_LIST_VARIABLE_SYMBOL_NAME,
            );

            nia_assert_is_err(&result);
        }
    }
}
