use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::function::builtin_function::{BuiltinFunctionType, BuiltinFunction};
use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::function::Function;
use crate::interpreter::function::special_form_function::{SpecialFormFunctionType, SpecialFormFunction};
use crate::interpreter::object::object::ObjectId;

pub fn infect_object_function(
    interpreter: &mut Interpreter,
    object_id: ObjectId,
    item_name: &str,
    func: BuiltinFunctionType
) -> Result<(), Error> {
    let name = interpreter.intern(item_name);

    let function = Function::Builtin(BuiltinFunction::new(func));
    let function_id = interpreter.register_function(function);
    let function_value = Value::Function(function_id);

    interpreter.set_object_item(
        object_id,
        name,
        function_value
    );

    Ok(())
}

pub fn infect_builtin_function(
    interpreter: &mut Interpreter,
    name: &str,
    func: BuiltinFunctionType
) -> Result<(), Error> {
    let name = interpreter.intern(name);

    let function = Function::Builtin(BuiltinFunction::new(func));
    let function_id = interpreter.register_function(function);
    let function_value = Value::Function(function_id);

    let result = interpreter.define_function(
        interpreter.get_root_environment(),
        name,
        function_value
    );

    match result {
        Ok(()) => Ok(()),
        Err(error) => Err(error)
    }
}

pub fn infect_special_form(
    interpreter: &mut Interpreter,
    name: &str,
    func: SpecialFormFunctionType
) -> Result<(), Error> {
    let name = interpreter.intern(name);

    let function = Function::SpecialForm(SpecialFormFunction::new(func));
    let function_id = interpreter.register_function(function);
    let function_value = Value::Function(function_id);

    let result = interpreter.define_function(
        interpreter.get_root_environment(),
        name,
        function_value
    );

    match result {
        Ok(()) => Ok(()),
        Err(error) => Err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod infect_special_form {
        use super::*;
        use crate::interpreter::environment::environment_arena::EnvironmentId;

        fn test(interpreter: &mut Interpreter, _environment: EnvironmentId, _values: Vec<Value>) -> Result<Value, Error>{
            Ok(interpreter.intern_nil_symbol_value())
        }

        #[test]
        fn sets_function() {
            let mut interpreter = Interpreter::raw();

            infect_special_form(&mut interpreter, "test", test).unwrap();

            let name = interpreter.intern("test");
            assert!(interpreter.has_function(interpreter.get_root_environment(), name));
        }

        #[test]
        fn returns_err_when_special_form_already_infected() {
            let mut interpreter = Interpreter::raw();

            infect_special_form(&mut interpreter, "test", test).unwrap();

            assert!(infect_special_form(&mut interpreter, "test", test).is_err());
        }
    }
}
