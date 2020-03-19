use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

pub fn start_listening(
    _interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    _values: Vec<Value>
) -> Result<Value, Error> {
    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
}
