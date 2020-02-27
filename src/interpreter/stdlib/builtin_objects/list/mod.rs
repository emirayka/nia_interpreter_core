use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::lib::infect::infect_object_builtin_function;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let list_object_id = interpreter.make_object();

    let list_symbol_id = interpreter.intern("list");

    interpreter.define_variable(
        interpreter.get_root_environment(),
        list_symbol_id,
        Value::Object(list_object_id)
    )?;

    Ok(())
}
