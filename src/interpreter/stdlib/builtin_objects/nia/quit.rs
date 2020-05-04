use std::process::exit;

use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn quit(
    _interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    _values: Vec<Value>,
) -> Result<Value, Error> {
    exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;
}
