use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod math;
mod symbol;
mod object;
mod list;
mod is;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    math::infect(interpreter)?;
    symbol::infect(interpreter)?;
    object::infect(interpreter)?;
    list::infect(interpreter)?;
    is::infect(interpreter)?;

    Ok(())
}
