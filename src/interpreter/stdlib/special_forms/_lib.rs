use crate::interpreter::value::Value;
use crate::interpreter::function::Function;
use crate::interpreter::function::special_form_function::{SpecialFormFunction, SpecialFormFunctionType};
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;

pub fn infect_special_form(
    interpreter: &mut Interpreter,
    name: &str,
    func: SpecialFormFunctionType
) -> Result<(), Error> {
    let name = interpreter.intern_symbol(name);

    let result = interpreter.define_function(
        interpreter.get_root_environment(),
        &name,
        Value::Function(Function::SpecialForm(SpecialFormFunction::new(func)))
    );

    match result {
        Ok(()) => Ok(()),
        Err(error) => Err(error)
    }
}


pub fn execute_forms(
    interpreter: &mut Interpreter,
    execution_environment: EnvironmentId,
    forms: Vec<Value>
) -> Result<Value, Error> {
    let mut last_result = None;

    for form in forms {
        let result = interpreter.execute_value(execution_environment, &form)?;
        last_result = Some(result);
    }

    match last_result {
        Some(value) => Ok(value),
        None => Ok(interpreter.intern_nil())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::environment::EnvironmentId;

    fn test(interpreter: &mut Interpreter, environment: EnvironmentId, values: Vec<Value>) -> Result<Value, Error>{
        Ok(interpreter.intern_nil())
    }

    #[test]
    fn test_sets_function() {
        let mut interpreter = Interpreter::raw();

        infect_special_form(&mut interpreter, "test", test).unwrap();

        let name = interpreter.intern_symbol("test");
        assert!(interpreter.has_function(interpreter.get_root_environment(), &name));
    }

    #[test]
    fn test_returns_err_when_special_form_already_infected() {
        let mut interpreter = Interpreter::raw();

        infect_special_form(&mut interpreter, "test", test).unwrap();

        assert!(infect_special_form(&mut interpreter, "test", test).is_err());
    }
}

