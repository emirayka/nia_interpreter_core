use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::stdlib::special_forms::_lib::infect_special_form;

fn _let(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    Ok(interpreter.intern_nil())
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_special_form(interpreter, "let", _let)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::error::assertion;
}
