use crate::interpreter::symbol::SymbolId;
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

    interpreter.set_variable(
        root_environment,
        symbol_name,
        value
    )
}

// todo: tests
