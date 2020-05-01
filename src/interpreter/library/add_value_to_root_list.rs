use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

use super::check_value_is_list;

pub fn add_value_to_root_list(
    interpreter: &mut Interpreter,
    name: &str,
    value: Value,
) -> Result<(), Error> {
    let root_environment = interpreter.get_root_environment();
    let symbol_name = interpreter.intern(name);

    let root_variable = interpreter.lookup_variable(
        root_environment,
        symbol_name,
    )?.ok_or_else(|| Error::generic_execution_error(
        "Cannot find variable."
    ))?;

    check_value_is_list(interpreter, root_variable)?;

    let new_cons = interpreter.make_cons_value(
        value,
        root_variable,
    );

    interpreter.set_variable(
        root_environment,
        symbol_name,
        new_cons,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::interpreter::library::assertion;

    const EMPTY_LIST_VARIABLE_SYMBOL_NAME: &'static str = "test-list-symbol";

    fn setup() -> Interpreter {
        let mut interpreter = Interpreter::new();

        let root_environment_id = interpreter.get_root_environment();
        let symbol = interpreter.intern(EMPTY_LIST_VARIABLE_SYMBOL_NAME);
        let nil = interpreter.intern_nil_symbol_value();

        interpreter.define_variable(
            root_environment_id,
            symbol,
            nil,
        ).unwrap();

        interpreter
    }

    #[test]
    fn adds_value_to_empty_list() {
        let mut interpreter = setup();
        let root_environment_id = interpreter.get_root_environment();
        let symbol = interpreter.intern(EMPTY_LIST_VARIABLE_SYMBOL_NAME);

        add_value_to_root_list(
            &mut interpreter,
            EMPTY_LIST_VARIABLE_SYMBOL_NAME,
            Value::Integer(1),
        ).unwrap();

        let result = interpreter.lookup_variable(
            root_environment_id,
            symbol,
        ).unwrap().ok_or_else(|| Error::generic_execution_error(
            "Cannot find variable."
        )).unwrap();

        let expected = interpreter.vec_to_list(vec!(Value::Integer(1)));

        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result,
        )
    }

    #[test]
    fn adds_value_to_list() {
        let mut interpreter = setup();

        let root_environment_id = interpreter.get_root_environment();
        let symbol = interpreter.intern(EMPTY_LIST_VARIABLE_SYMBOL_NAME);

        add_value_to_root_list(
            &mut interpreter,
            EMPTY_LIST_VARIABLE_SYMBOL_NAME,
            Value::Integer(1),
        ).unwrap();

        add_value_to_root_list(
            &mut interpreter,
            EMPTY_LIST_VARIABLE_SYMBOL_NAME,
            Value::Integer(2),
        ).unwrap();

        let result = interpreter.lookup_variable(
            root_environment_id,
            symbol,
        ).unwrap().ok_or_else(|| Error::generic_execution_error(
            "Cannot find variable."
        )).unwrap();

        let expected = interpreter.vec_to_list(
            vec!(Value::Integer(2), Value::Integer(1))
        );

        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result,
        )
    }
}