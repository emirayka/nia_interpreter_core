use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::function::builtin_function::{BuiltinFunction, BuiltinFunctionType};
use crate::interpreter::function::Function;
use crate::interpreter::value::Value;
use crate::interpreter::object::ObjectId;

mod object_make;
mod object_get;
mod object_set;
mod object_get_proto;
mod object_set_proto;

fn infect_to_object(
    interpreter: &mut Interpreter,
    object_id: ObjectId,
    item_name: &str,
    func: BuiltinFunctionType
) -> Result<(), Error> {
    let name = interpreter.intern_symbol(item_name);

    interpreter.set_object_item(
        object_id,
        &name,
        Value::Function(Function::Builtin(BuiltinFunction::new(func)))
    );

    Ok(())
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let object_object_id = interpreter.make_object();

    infect_to_object(
        interpreter,
        object_object_id,
        "make",
        object_make::object_make
    )?;

    infect_to_object(
        interpreter,
        object_object_id,
        "get",
        object_get::object_get
    )?;

    infect_to_object(
        interpreter,
        object_object_id,
        "set!",
        object_set::object_set
    )?;

    let name = interpreter.intern_symbol("object");

    interpreter.define_variable(
        interpreter.get_root_environment(),
        &name,
        Value::Object(object_object_id)
    )?;

    Ok(())
}
