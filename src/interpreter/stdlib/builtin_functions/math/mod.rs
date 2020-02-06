use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod sum;
mod sub;
mod mul;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    sum::infect(interpreter)?;
    sub::infect(interpreter)?;
    mul::infect(interpreter)?;

    Ok(())
}
