use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn evaluate_forms(
    interpreter: &mut Interpreter,
    execution_environment: EnvironmentId,
    forms: Vec<Value>,
) -> Result<Vec<Value>, Error> {
    let mut results = Vec::new();

    for form in forms {
        let result = interpreter.execute_value(execution_environment, form)?;

        results.push(result);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_the_execution_result_of_forms() {
        let mut interpreter = Interpreter::new();

        let main_environment_id = interpreter.get_main_environment_id();
        let symbol_id = interpreter.intern_symbol_id("test");

        interpreter
            .define_variable(
                interpreter.get_root_environment_id(),
                symbol_id,
                Value::Integer(10),
            )
            .unwrap();

        let forms = vec![Value::Integer(1), Value::Symbol(symbol_id)];
        let expected = vec![Value::Integer(1), Value::Integer(10)];

        let result =
            evaluate_forms(&mut interpreter, main_environment_id, forms)
                .unwrap();

        crate::utils::assert_vectors_deep_equal(
            &mut interpreter,
            expected,
            result,
        )
    }

    #[test]
    fn returns_err_when_execution_failed() {
        let mut interpreter = Interpreter::new();
        let root_environment_id = interpreter.get_root_environment_id();

        let forms =
            vec![Value::Integer(1), interpreter.intern_symbol_value("test")];

        let result =
            evaluate_forms(&mut interpreter, root_environment_id, forms);

        nia_assert_is_err(&result);
    }
}
