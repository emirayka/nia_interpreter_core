use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::BuiltinFunctionType;
use crate::interpreter::value::Value;

use crate::library;

mod define;
mod define_global_mapping;
mod define_modifier;
mod is_listening_question;
mod start_listening;
mod stop_listening;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let device_object_id = interpreter.make_object();

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec![
        (
            "define-global-mapping",
            define_global_mapping::define_global_mapping,
        ),
        ("define-modifier", define_modifier::define_modifier),
        ("define", define::define),
        ("start-listening", start_listening::start_listening),
        ("stop-listening", stop_listening::stop_listening),
        (
            "is-listening?",
            is_listening_question::is_listening_question,
        ),
    ];

    for (name, func) in bindings {
        library::infect_object_builtin_function(
            interpreter,
            device_object_id,
            name,
            func,
        )?;
    }

    let device_symbol_id = interpreter.intern_symbol_id("device");

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        device_symbol_id,
        Value::Object(device_object_id),
    )?;

    Ok(())
}
