use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::{library, DEFINED_DEVICES_ROOT_VARIABLE_NAME};

pub fn start_listening(
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

    let devices_list = library::get_root_variable(
        interpreter,
        DEFINED_DEVICES_ROOT_VARIABLE_NAME,
    )?;

    let devices = library::read_as_vector(interpreter, devices_list)?;

    if devices.len() == 0 {
        return Error::generic_execution_error("No devices were defined.")
            .into();
    }

    interpreter.start_listening();

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
    }

    #[test]
    fn returns_generic_execution_error_when_no_devices_were_defined() {
        let mut interpreter = Interpreter::new();

        nia_assert(!interpreter.is_listening());

        let result = interpreter
            .execute_in_main_environment(r#"(device:start-listening)"#);

        crate::utils::assert_generic_execution_error(&result);
    }
}
