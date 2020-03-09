use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

pub fn execute_forms(
    interpreter: &mut Interpreter,
    execution_environment: EnvironmentId,
    forms: Vec<Value>
) -> Result<Value, Error> {
    let mut last_result = None;

    for form in forms {
        let result = interpreter.execute_value(execution_environment, form)?;
        last_result = Some(result);
    }

    match last_result {
        Some(value) => Ok(value),
        None => Ok(interpreter.intern_nil_symbol_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    // todo: ensure this test is fine
    #[test]
    fn returns_the_result_of_execution_of_the_last_form() {
        let mut interpreter = Interpreter::new();
        let symbol_id = interpreter.intern("test");

        interpreter.define_variable(
            interpreter.get_root_environment(),
            symbol_id,
            Value::Integer(10)
        ).unwrap();

        let forms = vec!(
            Value::Integer(1),
            Value::Symbol(symbol_id)
        );

        let root_env = interpreter.get_root_environment();

        let result = execute_forms(
            &mut interpreter,
            root_env,
            forms
        );

        assert_eq!(Value::Integer(10), result.unwrap());
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_err_when_execution_failed() {
        let mut interpreter = Interpreter::new();

        let forms = vec!(
            Value::Integer(1),
            interpreter.intern_symbol_value("test")
        );

        let root_env = interpreter.get_root_environment();

        let result = execute_forms(
            &mut interpreter,
            root_env,
            forms
        );

        assertion::assert_is_error(&result);
    }
}
