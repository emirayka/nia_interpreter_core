use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::BuiltinFunctionType;
use crate::interpreter::value::Value;

use crate::library;

mod abs;
mod ceil;
mod floor;
mod max;
mod min;
mod pow;
mod round;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let math_symbol_id = interpreter.intern_symbol_id("math");
    let math_object_id = interpreter.make_object();

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec![
        ("abs", abs::abs),
        ("ceil", ceil::ceil),
        ("floor", floor::floor),
        ("max", max::max),
        ("min", min::min),
        ("pow", pow::pow),
        ("round", round::round),
    ];

    for (name, func) in bindings {
        library::infect_object_builtin_function(
            interpreter,
            math_object_id,
            name,
            func,
        )?;
    }

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        math_symbol_id,
        Value::Object(math_object_id),
    )?;

    Ok(())
}
