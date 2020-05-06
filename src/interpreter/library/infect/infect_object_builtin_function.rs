use crate::BuiltinFunction;
use crate::BuiltinFunctionType;
use crate::Error;
use crate::Function;
use crate::Interpreter;
use crate::ObjectId;
use crate::Value;

pub fn infect_object_builtin_function(
    interpreter: &mut Interpreter,
    object_id: ObjectId,
    item_name: &str,
    func: BuiltinFunctionType,
) -> Result<(), Error> {
    let name = interpreter.intern_symbol_id(item_name);

    let function = Function::Builtin(BuiltinFunction::new(func));
    let function_id = interpreter.register_function(function);
    let function_value = Value::Function(function_id);

    interpreter.set_object_property(object_id, name, function_value)?;

    Ok(())
}
