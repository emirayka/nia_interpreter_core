use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::error::Error;
use crate::interpreter::value::Value;

mod special_forms;
mod builtin_functions;

pub fn infect_stdlib(interpreter: &mut Interpreter) -> Result<(), Error> {
    special_forms::infect(interpreter)?;
    builtin_functions::infect(interpreter)?;

    let root = interpreter.get_root_environment();
    let nil_value = interpreter.intern_nil_symbol_value();

    if let Value::Symbol(symbol_id) = nil_value {
        interpreter.define_variable(root, symbol_id, nil_value)?;
    } else {
        panic!()
    }

    Ok(())
}