use crate::Error;
use crate::Function;
use crate::Interpreter;
use crate::SpecialFormFunction;
use crate::SpecialFormFunctionType;
use crate::Value;

pub fn infect_special_form(
    interpreter: &mut Interpreter,
    name: &str,
    func: SpecialFormFunctionType,
) -> Result<(), Error> {
    let name = interpreter.intern_symbol_id(name);

    let function = Function::SpecialForm(SpecialFormFunction::new(func));
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

#[cfg(test)]
mod infect_special_form {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::interpreter::environment::EnvironmentId;

    fn test(
        interpreter: &mut Interpreter,
        _environment: EnvironmentId,
        _values: Vec<Value>,
    ) -> Result<Value, Error> {
        Ok(interpreter.intern_nil_symbol_value())
    }

    #[test]
    fn sets_function() {
        let mut interpreter = Interpreter::raw();
        let root_environment_id = interpreter.get_root_environment_id();

        infect_special_form(&mut interpreter, "test", test).unwrap();

        let function_symbol_id = interpreter.intern_symbol_id("test");
        nia_assert(
            interpreter
                .has_function(root_environment_id, function_symbol_id)
                .unwrap(),
        );
    }

    #[test]
    fn returns_err_when_special_form_already_infected() {
        let mut interpreter = Interpreter::raw();

        infect_special_form(&mut interpreter, "test", test).unwrap();

        nia_assert(
            infect_special_form(&mut interpreter, "test", test).is_err(),
        );
    }
}
