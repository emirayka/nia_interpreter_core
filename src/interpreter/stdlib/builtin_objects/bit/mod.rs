use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library::infect::infect_object_builtin_function;
use crate::interpreter::value::BuiltinFunctionType;

mod and;
mod clear;
mod flip;
mod not;
mod or;
mod set;
mod shift_left;
mod shift_right;
mod test;
mod xor;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let bit_object_id = interpreter.make_object();
    let bit_symbol_id = interpreter.intern_symbol_id("bit");

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec![
        ("and", and::and),
        ("clear", clear::clear),
        ("flip", flip::flip),
        ("not", not::not),
        ("or", or::or),
        ("set", set::set),
        ("shift-left", shift_left::shift_left),
        ("shift-right", shift_right::shift_right),
        ("test", test::test),
        ("xor", xor::xor),
    ];

    for (name, func) in bindings {
        infect_object_builtin_function(interpreter, bit_object_id, name, func)?;
    }

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        bit_symbol_id,
        Value::Object(bit_object_id),
    )?;

    Ok(())
}
