use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::BuiltinFunctionType;
use crate::interpreter::value::Value;

use crate::library;

mod acons;
mod acons_mark;
mod has_key_question;
mod has_value_question;
mod lookup;
mod new;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let alist_object_id = interpreter.make_object();
    let alist_symbol_id = interpreter.intern_symbol_id("alist");

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec![
        ("acons", acons::acons),
        ("acons!", acons_mark::acons_mark),
        ("has-key?", has_key_question::has_key_question),
        ("has-value?", has_value_question::has_value_question),
        ("lookup", lookup::lookup),
        ("new", new::new),
    ];

    for (name, func) in bindings {
        library::infect_object_builtin_function(
            interpreter,
            alist_object_id,
            name,
            func,
        )?;
    }

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        alist_symbol_id,
        Value::Object(alist_object_id),
    )?;

    Ok(())
}
