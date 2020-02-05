use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::function::builtin_function::{BuiltinFunctionType, BuiltinFunction};
use crate::interpreter::function::Function;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

pub fn infect_builtin_function(
    interpreter: &mut Interpreter,
    name: &str,
    func: BuiltinFunctionType
) -> Result<(), Error> {
    let name = interpreter.intern_symbol(name);

    let result = interpreter.define_function(
        interpreter.get_root_environment(),
        &name,
        Value::Function(Function::Builtin(BuiltinFunction::new(func)))
    );

    match result {
        Ok(()) => Ok(()),
        Err(error) => Err(error)
    }
}
