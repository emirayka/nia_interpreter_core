use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod lang;
mod func;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    lang::infect(interpreter)?;
    func::infect(interpreter)?;

    Ok(())
}
