use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::BuiltinFunctionType;
use crate::interpreter::value::Value;

use crate::library;

mod define_global_mapping;
mod define_modifier;
mod register;
mod start_listening;
mod stop_listening;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let keyboard_object_id = interpreter.make_object();

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec![
        (
            "define-global-mapping",
            define_global_mapping::define_global_mapping,
        ),
        ("define-modifier", define_modifier::define_modifier),
        ("register", register::register),
        ("start-listening", start_listening::start_listening),
        ("stop-listening", stop_listening::stop_listening),
    ];

    for (name, func) in bindings {
        library::infect_object_builtin_function(
            interpreter,
            keyboard_object_id,
            name,
            func,
        )?;
    }

    let keyboard_symbol_id = interpreter.intern_symbol_id("keyboard");

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        keyboard_symbol_id,
        Value::Object(keyboard_object_id),
    )?;

    Ok(())
}
