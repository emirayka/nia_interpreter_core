use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::stdlib::builtin_functions::_lib::infect_builtin_function;

fn sum(
    interpreter: &mut Interpreter,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() < 2 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Function `sum' must take at least two arguments"
        ));
    }

    return Ok(Value::Integer(0));
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_builtin_function(interpreter, "+", sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::cons::Cons;
    use crate::interpreter::error::assertion;

}
