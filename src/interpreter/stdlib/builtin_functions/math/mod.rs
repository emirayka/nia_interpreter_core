use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod sum;
mod sub;
mod mul;
mod div;
mod rem;
mod pow;

mod floor;
mod round;
mod ceil;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    sum::infect(interpreter)?;
    sub::infect(interpreter)?;
    mul::infect(interpreter)?;
    rem::infect(interpreter)?;

    Ok(())
}
