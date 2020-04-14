use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library::infect::{infect_object_builtin_function};
use crate::interpreter::value::Value;
use crate::interpreter::function::BuiltinFunctionType;

mod define_global_mapping;
mod define_modifier;
mod register;
mod start_listening;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let keyboard_object_id = interpreter.make_object();

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec!(
        ("register", register::register),
        ("start-listening", start_listening::start_listening),
        ("define-global-mapping", define_global_mapping::define_global_mapping),
        ("define-modifier", define_modifier::define_modifier),
    );

    for (name, func) in bindings {
        infect_object_builtin_function(
            interpreter,
            keyboard_object_id,
            name,
            func
        )?;
    }

    let keyboard_symbol_id = interpreter.intern("keyboard");

    interpreter.define_variable(
        interpreter.get_root_environment(),
        keyboard_symbol_id,
        Value::Object(keyboard_object_id)
    )?;

    Ok(())
}
