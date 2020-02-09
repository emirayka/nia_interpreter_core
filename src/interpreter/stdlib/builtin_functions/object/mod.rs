use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::function::builtin_function::{BuiltinFunction, BuiltinFunctionType};
use crate::interpreter::function::Function;
use crate::interpreter::value::Value;
use crate::interpreter::object::ObjectId;

mod make_object;

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
        make_object::make_object
    )?;

    let name = interpreter.intern_symbol("object");

    interpreter.define_variable(
        interpreter.get_root_environment(),
        &name,
        Value::Object(object_object_id)
    )?;

    Ok(())
}
