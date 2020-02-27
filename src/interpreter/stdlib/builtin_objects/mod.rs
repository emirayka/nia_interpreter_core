use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod is;
mod list;
mod math;
mod object;
mod string;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    is::infect(interpreter)?;
    list::infect(interpreter)?;
    math::infect(interpreter)?;
    object::infect(interpreter)?;
    string::infect(interpreter)?;

    Ok(())
}
