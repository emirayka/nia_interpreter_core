use std::process::exit;

use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

pub fn quit(
    _interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    _values: Vec<Value>
) -> Result<Value, Error> {
    exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;
    use crate::interpreter::library::testing_helpers::for_special_symbols;
}
