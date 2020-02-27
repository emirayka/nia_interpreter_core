use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod math;
mod object;
mod is;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    math::infect(interpreter)?;
    object::infect(interpreter)?;
    is::infect(interpreter)?;

    Ok(())
}
