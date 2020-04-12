use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

pub fn register(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 0 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `keyboard:register' takes zero arguments exactly."
        ).into_result()
    }

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {
}
