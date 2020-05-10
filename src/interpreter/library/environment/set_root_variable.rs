use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::SymbolId;
use crate::interpreter::value::Value;

pub fn set_root_variable(
    interpreter: &mut Interpreter,
    name: &str,
    value: Value,
) -> Result<(), Error> {
    let root_environment = interpreter.get_root_environment_id();
    let symbol_name = interpreter.intern_symbol_id(name);

    if interpreter.has_variable(root_environment, symbol_name)? {
        interpreter.set_variable(root_environment, symbol_name, value)
    } else {
        interpreter.define_variable(root_environment, symbol_name, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    const VARIABLE_SYMBOL_NAME: &'static str = "test-symbol";

    #[test]
    fn sets_already_defined_variable() {
        let mut interpreter = Interpreter::new();

        let root_environment_id = interpreter.get_root_environment_id();
        let symbol = interpreter.intern_symbol_id(VARIABLE_SYMBOL_NAME);

        interpreter
            .define_variable(root_environment_id, symbol, Value::Integer(0))
            .unwrap();

        let expected = Value::Integer(1);
        set_root_variable(&mut interpreter, VARIABLE_SYMBOL_NAME, expected)
            .unwrap();

        let result = interpreter
            .lookup_variable(root_environment_id, symbol)
            .unwrap()
            .unwrap();

        utils::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn defines_variable_if_it_is_not_defined_yet() {
        let mut interpreter = Interpreter::new();

        let root_environment_id = interpreter.get_root_environment_id();
        let symbol = interpreter.intern_symbol_id(VARIABLE_SYMBOL_NAME);

        let expected = Value::Integer(1);
        set_root_variable(&mut interpreter, VARIABLE_SYMBOL_NAME, expected)
            .unwrap();

        let result = interpreter
            .lookup_variable(root_environment_id, symbol)
            .unwrap()
            .unwrap();

        utils::assert_deep_equal(&mut interpreter, expected, result);
    }
}
