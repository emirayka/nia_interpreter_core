use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::BuiltinFunctionType;
use crate::interpreter::value::Value;

use crate::library;

mod and;
mod nand;
mod nor;
mod or;
mod xor;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let logic_object_id = interpreter.make_object();

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec![
        ("and", and::and),
        ("nand", nand::nand),
        ("nor", nor::nor),
        ("or", or::or),
        ("xor", xor::xor),
    ];

    for (name, func) in bindings {
        library::infect_object_builtin_function(
            interpreter,
            logic_object_id,
            name,
            func,
        )?;
    }

    let logic_symbol_id = interpreter.intern_symbol_id("logic");

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        logic_symbol_id,
        Value::Object(logic_object_id),
    )?;

    Ok(())
}
