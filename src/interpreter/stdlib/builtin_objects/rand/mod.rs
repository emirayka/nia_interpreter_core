use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library::infect::infect_object_builtin_function;
use crate::interpreter::value::BuiltinFunctionType;
use crate::interpreter::value::Value;

mod float;
mod int;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let rand_object_id = interpreter.make_object();

    let bindings: Vec<(&str, BuiltinFunctionType)> =
        vec![("int", int::int), ("float", float::float)];

    for (name, func) in bindings {
        infect_object_builtin_function(interpreter, rand_object_id, name, func)?;
    }

    let rand_symbol_id = interpreter.intern("rand");

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        rand_symbol_id,
        Value::Object(rand_object_id),
    )?;

    Ok(())
}
