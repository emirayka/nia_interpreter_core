use std::collections::HashMap;

use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn stop_listening(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 0 {
        return Error::invalid_argument_count_error(
            "Built-in function `keyboard:start-listening' takes no arguments."
        ).into();
    }

    interpreter.stop_listening();

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    // #[test]
    // fn simple_test() {
    //     let mut interpreter = Interpreter::new();
    //
    //     interpreter.execute(r#"(keyboard:register "/dev/input/event6" "first") (keyboard:start-listening)"#).unwrap();
    // }
}
