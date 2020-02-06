use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod _lib;

mod math;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    math::infect(interpreter)?;

    Ok(())
}
