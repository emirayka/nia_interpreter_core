use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod sum;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    sum::infect(interpreter)?;

    Ok(())
}
