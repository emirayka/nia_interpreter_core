use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library::infect::infect_object_builtin_function;
use crate::interpreter::value::BuiltinFunctionType;

mod quit;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let nia_symbol_id = interpreter.intern_symbol_id("nia");
    let nia_object_id = interpreter.make_object();

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec![("quit", quit::quit)];

    for (name, func) in bindings {
        infect_object_builtin_function(interpreter, nia_object_id, name, func)?;
    }

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        nia_symbol_id,
        Value::Object(nia_object_id),
    )?;

    Ok(())
}
