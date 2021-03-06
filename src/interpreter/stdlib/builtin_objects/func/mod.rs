use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::BuiltinFunctionType;
use crate::interpreter::value::Value;

use crate::library;

mod always;
mod apply;
mod call;
mod combine;
mod f;
mod id;
mod t;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let func_object_id = interpreter.make_object();

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec![
        ("always", always::always),
        ("apply", apply::apply),
        ("call", call::call),
        ("combine", combine::combine),
        ("id", id::id),
        ("f", f::f),
        ("t", t::t),
    ];

    for (name, func) in bindings {
        library::infect_object_builtin_function(
            interpreter,
            func_object_id,
            name,
            func,
        )?;
    }

    let func_symbol_id = interpreter.intern_symbol_id("func");

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        func_symbol_id,
        Value::Object(func_object_id),
    )?;

    Ok(())
}
