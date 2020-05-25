use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn stop_listening(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 0 {
        return Error::invalid_argument_count_error(
            "Built-in function `device:start-listening' takes no arguments.",
        )
        .into();
    }

    interpreter.stop_listening();

    Ok(interpreter.intern_nil_symbol_value())
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

        nia_assert(!interpreter.is_listening());

        interpreter
            .execute_in_main_environment(r#"(device:define 0 "/dev/input/event6" "first") (device:start-listening)"#)
            .unwrap();

        nia_assert(interpreter.is_listening());

        interpreter
            .execute_in_main_environment(r#"(device:stop-listening)"#)
            .unwrap();

        nia_assert(!interpreter.is_listening());
    }
}
