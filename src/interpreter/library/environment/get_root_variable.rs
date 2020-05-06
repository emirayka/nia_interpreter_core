use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn get_root_variable(
    interpreter: &mut Interpreter,
    name: &str,
) -> Result<Value, Error> {
    let root_environment = interpreter.get_root_environment_id();
    let symbol_name = interpreter.intern_symbol_id(name);

    interpreter
        .lookup_variable(root_environment, symbol_name)?
        .ok_or_else(|| Error::generic_execution_error("Cannot find variable."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    const VARIABLE_SYMBOL_NAME: &'static str = "test-symbol";

    #[test]
    fn returns_variable_in_root_environment() {
        let mut interpreter = Interpreter::new();

        let root_environment_id = interpreter.get_root_environment_id();
        let symbol = interpreter.intern_symbol_id(VARIABLE_SYMBOL_NAME);

        interpreter
            .define_variable(root_environment_id, symbol, Value::Integer(1))
            .unwrap();

        let expected = interpreter
            .lookup_variable(root_environment_id, symbol)
            .unwrap()
            .unwrap();

        let result =
            get_root_variable(&mut interpreter, VARIABLE_SYMBOL_NAME).unwrap();

        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }
}
