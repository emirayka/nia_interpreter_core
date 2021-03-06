use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::BuiltinFunctionType;
use crate::interpreter::value::Value;

use crate::library;

mod boolean;
mod float;
mod int;
mod keyword;
mod string;
mod symbol;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let to_object_id = interpreter.make_object();
    let to_symbol_id = interpreter.intern_symbol_id("to");

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec![
        ("boolean", boolean::boolean),
        ("float", float::float),
        ("int", int::int),
        ("keyword", keyword::keyword),
        ("string", string::string),
        ("symbol", symbol::symbol),
    ];

    for (name, func) in bindings {
        library::infect_object_builtin_function(
            interpreter,
            to_object_id,
            name,
            func,
        )?;
    }

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        to_symbol_id,
        Value::Object(to_object_id),
    )?;

    Ok(())
}
