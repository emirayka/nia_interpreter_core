use crate::interpreter::value::SymbolId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

pub fn set_root_variable(
    interpreter: &mut Interpreter,
    name: &str,
    value: Value
) -> Result<(), Error> {
    let root_environment = interpreter.get_root_environment();
    let symbol_name = interpreter.intern(name);

    if interpreter.has_variable(
        root_environment,
        symbol_name
    )? {
        interpreter.set_variable(
            root_environment,
            symbol_name,
            value
        )
    } else {
        interpreter.define_variable(
            root_environment,
            symbol_name,
            value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::interpreter::library::assertion;

    const VARIABLE_SYMBOL_NAME: &'static str = "test-symbol";

    #[test]
    fn sets_already_defined_variable() {
        let mut interpreter = Interpreter::new();

        let root_environment_id = interpreter.get_root_environment();
        let symbol = interpreter.intern(VARIABLE_SYMBOL_NAME);

        interpreter.define_variable(
            root_environment_id,
            symbol,
            Value::Integer(0)
        ).unwrap();

        let expected = Value::Integer(1);
        set_root_variable(
            &mut interpreter,
            VARIABLE_SYMBOL_NAME,
            expected
        ).unwrap();

        let result = interpreter.lookup_variable(
            root_environment_id,
            symbol
        ).unwrap().unwrap();

        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result
        );
    }

    #[test]
    fn defines_variable_if_it_is_not_defined_yet() {
        let mut interpreter = Interpreter::new();

        let root_environment_id = interpreter.get_root_environment();
        let symbol = interpreter.intern(VARIABLE_SYMBOL_NAME);

        let expected = Value::Integer(1);
        set_root_variable(
            &mut interpreter,
            VARIABLE_SYMBOL_NAME,
            expected
        ).unwrap();

        let result = interpreter.lookup_variable(
            root_environment_id,
            symbol
        ).unwrap().unwrap();

        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result
        );
    }
}

