use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod intern;
mod gensym;
mod lookup;
mod flookup;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    intern::infect(interpreter)?;
    gensym::infect(interpreter)?;

    Ok(())
}
