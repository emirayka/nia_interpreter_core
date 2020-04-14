use crate::interpreter::symbol::SymbolId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

pub fn get_root_variable(
    interpreter: &mut Interpreter,
    name: &str
) -> Result<Value, Error> {
    let root_environment = interpreter.get_root_environment();
    let symbol_name = interpreter.intern(name);

    interpreter.lookup_variable(
        root_environment,
        symbol_name
    )
}

// todo: tests
