use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod func;
mod lang;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    lang::infect(interpreter)?;
    func::infect(interpreter)?;

    Ok(())
}
