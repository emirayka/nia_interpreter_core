use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

pub fn start_listening(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use crate::interpreter::library::assertion;
}
