use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn execute_forms(
    interpreter: &mut Interpreter,
    execution_environment: EnvironmentId,
    forms: &Vec<Value>,
) -> Result<Value, Error> {
    let mut last_result = None;

    for form in forms {
        let result = interpreter.execute_value(execution_environment, *form)?;
        last_result = Some(result);
    }

    match last_result {
        Some(value) => Ok(value),
        None => Ok(interpreter.intern_nil_symbol_value()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_the_result_of_execution_of_the_last_form() {
        let mut interpreter = Interpreter::new();

        let root_environment_id = interpreter.get_root_environment_id();
        let symbol_id = interpreter.intern("test");

        interpreter
            .define_variable(
                interpreter.get_root_environment_id(),
                symbol_id,
                Value::Integer(10),
            )
            .unwrap();

        let forms = vec![Value::Integer(1), Value::Symbol(symbol_id)];

        let result = execute_forms(&mut interpreter, root_environment_id, &forms);

        nia_assert_equal(Value::Integer(10), result.unwrap());
    }

    #[test]
    fn returns_err_when_execution_failed() {
        let mut interpreter = Interpreter::new();
        let root_environment_id = interpreter.get_root_environment_id();

        let forms = vec![Value::Integer(1), interpreter.intern_symbol_value("test")];

        let result = execute_forms(&mut interpreter, root_environment_id, &forms);

        nia_assert_is_err(&result);
    }
}
