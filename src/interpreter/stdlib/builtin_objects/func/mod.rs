use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library::infect::{infect_object_builtin_function};
use crate::interpreter::value::Value;
use crate::interpreter::function::BuiltinFunctionType;

mod always;
mod combine;
mod curry;
mod id;
mod f;
mod t;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let func_object_id = interpreter.make_object();

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec!(
        ("always", always::always),
        ("combine", combine::combine),
        ("curry", curry::curry),
        ("id", id::id),
        ("f", f::f),
        ("t", t::t),
    );

    for (name, func) in bindings {
        infect_object_builtin_function(
            interpreter,
            func_object_id,
            name,
            func
        )?;
    }

    let func_symbol_id = interpreter.intern("func");

    interpreter.define_variable(
        interpreter.get_root_environment(),
        func_symbol_id,
        Value::Object(func_object_id)
    )?;

    Ok(())
}
