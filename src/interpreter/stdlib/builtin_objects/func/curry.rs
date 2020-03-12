use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::function::{
    FunctionId,
};
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

fn curry_interpreted_function(
    interpreter: &mut Interpreter,
    function_id: FunctionId,
    argument_count: Option<i64>
) -> Result<Value, Error> {
    unimplemented!()
}

fn curry_builtin_function(
    interpreter: &mut Interpreter,
    function_id: FunctionId,
    argument_count: Option<i64>
) -> Result<Value, Error> {
    let argument_count = match argument_count {
        Some(argument_count) => argument_count,
        _ => return interpreter.make_invalid_argument_count_error(
            "Built-in function `func:curry' cannot curry a built-in function, if argument count is unknown."
        ).into_result()
    };
    unimplemented!()
}

pub fn curry(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;
}
