use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library::infect::infect_object_builtin_function;

mod ceil;
mod floor;
mod pow;
mod round;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let math_symbol_id = interpreter.intern("math");
    let math_object_id = interpreter.make_object();

    infect_object_builtin_function(
        interpreter,
        math_object_id,
        "ceil",
        ceil::ceil
    )?;

    infect_object_builtin_function(
        interpreter,
        math_object_id,
        "floor",
        floor::floor
    )?;

    infect_object_builtin_function(
        interpreter,
        math_object_id,
        "pow",
        pow::pow
    )?;

    infect_object_builtin_function(
        interpreter,
        math_object_id,
        "round",
        round::round
    )?;

    interpreter.define_variable(
        interpreter.get_root_environment(),
        math_symbol_id,
        Value::Object(math_object_id)
    )?;

    Ok(())
}
