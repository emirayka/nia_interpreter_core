use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library::infect::infect_object_builtin_function;

// mod define;
// mod delete;
// mod freeze_mark;
mod new;
mod make;
mod get;
mod set;
mod get_proto;
mod set_proto;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let object_object_id = interpreter.make_object();

    infect_object_builtin_function(
        interpreter,
        object_object_id,
        "make",
        make::make
    )?;

    infect_object_builtin_function(
        interpreter,
        object_object_id,
        "get",
        get::get
    )?;

    infect_object_builtin_function(
        interpreter,
        object_object_id,
        "set!",
        set::set
    )?;

    infect_object_builtin_function(
        interpreter,
        object_object_id,
        "get-proto",
        get_proto::object_get_proto
    )?;

    infect_object_builtin_function(
        interpreter,
        object_object_id,
        "set-proto!",
        set_proto::set_proto
    )?;

    infect_object_builtin_function(
        interpreter,
        object_object_id,
        "new",
        new::new
    )?;

    let object_symbol_id = interpreter.intern("object");

    interpreter.define_variable(
        interpreter.get_root_environment(),
        object_symbol_id,
        Value::Object(object_object_id)
    )?;

    Ok(())
}
