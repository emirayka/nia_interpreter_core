use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::lib::infect::infect_object;

mod object_new;
mod object_make;
mod object_get;
mod object_set;
mod object_get_proto;
mod object_set_proto;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let object_object_id = interpreter.make_object();

    infect_object(
        interpreter,
        object_object_id,
        "make",
        object_make::object_make
    )?;

    infect_object(
        interpreter,
        object_object_id,
        "get",
        object_get::object_get
    )?;

    infect_object(
        interpreter,
        object_object_id,
        "set!",
        object_set::object_set
    )?;

    infect_object(
        interpreter,
        object_object_id,
        "get-proto",
        object_get_proto::object_get_proto
    )?;

    infect_object(
        interpreter,
        object_object_id,
        "set-proto!",
        object_set_proto::object_set_proto
    )?;

    infect_object(
        interpreter,
        object_object_id,
        "new",
        object_new::object_new
    )?;

    let name = interpreter.intern_symbol("object");

    interpreter.define_variable(
        interpreter.get_root_environment(),
        &name,
        Value::Object(object_object_id)
    )?;

    Ok(())
}
