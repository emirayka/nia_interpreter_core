use crate::BuiltinFunction;
use crate::BuiltinFunctionType;
use crate::Error;
use crate::Function;
use crate::Interpreter;
use crate::Value;

pub fn infect_builtin_function(
    interpreter: &mut Interpreter,
    name: &str,
    func: BuiltinFunctionType,
) -> Result<(), Error> {
    let name = interpreter.intern_symbol_id(name);

    let function = Function::Builtin(BuiltinFunction::new(func));
    let function_id = interpreter.register_function(function);
    let function_value = Value::Function(function_id);

    let result = interpreter.define_function(
        interpreter.get_root_environment_id(),
        name,
        function_value,
    );

    match result {
        Ok(()) => Ok(()),
        Err(error) => Err(error),
    }
}
