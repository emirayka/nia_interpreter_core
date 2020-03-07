use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library::infect::infect_object_builtin_function;

mod object_new;
mod object_make;
mod object_get;
mod object_set;
mod object_get_proto;
mod object_set_proto;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let object_object_id = interpreter.make_object();

    infect_object_builtin_function(
        interpreter,
        object_object_id,
        "make",
        object_make::object_make
    )?;

    infect_object_builtin_function(
        interpreter,
        object_object_id,
        "get",
        object_get::object_get
    )?;

    infect_object_builtin_function(
        interpreter,
        object_object_id,
        "set!",
        object_set::object_set
    )?;

    infect_object_builtin_function(
        interpreter,
        object_object_id,
        "get-proto",
        object_get_proto::object_get_proto
    )?;

    infect_object_builtin_function(
        interpreter,
        object_object_id,
        "set-proto!",
        object_set_proto::object_set_proto
    )?;

    infect_object_builtin_function(
        interpreter,
        object_object_id,
        "new",
        object_new::object_new
    )?;

    let object_symbol_id = interpreter.intern("object");

    interpreter.define_variable(
        interpreter.get_root_environment(),
        object_symbol_id,
        Value::Object(object_object_id)
    )?;

    Ok(())
}
