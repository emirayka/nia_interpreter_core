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
mod ceiling;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    sum::infect(interpreter)?;
    sub::infect(interpreter)?;
    mul::infect(interpreter)?;
    div::infect(interpreter)?;
    rem::infect(interpreter)?;
    pow::infect(interpreter)?;

    floor::infect(interpreter)?;
    round::infect(interpreter)?;
    ceiling::infect(interpreter)?;

    Ok(())
}
