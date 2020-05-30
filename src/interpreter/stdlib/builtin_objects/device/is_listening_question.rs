use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn is_listening_question(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 0 {
        return Error::invalid_argument_count_error(
            "Built-in function `device:is-listening?' takes no arguments.",
        )
        .into();
    }

    let is_listening = interpreter.is_listening();

    Ok(Value::Boolean(is_listening))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn simple_test() {
        let mut interpreter = Interpreter::new();

        let result = interpreter
            .execute_in_main_environment(r#"(device:is-listening?)"#)
            .unwrap();
        let expected = Value::Boolean(false);
        crate::utils::assert_deep_equal(&mut interpreter, expected, result);

        nia_assert_is_ok(&interpreter.start_listening());

        let result = interpreter
            .execute_in_main_environment(r#"(device:is-listening?)"#)
            .unwrap();
        let expected = Value::Boolean(true);
        crate::utils::assert_deep_equal(&mut interpreter, expected, result);
    }
}
