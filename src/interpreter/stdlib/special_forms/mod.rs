use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod cond;
mod quote;
mod define_variable;

pub fn infect_special_forms(interpreter: &mut Interpreter) -> Result<(), Error> {
    cond::infect(interpreter)?;
    quote::infect(interpreter)?;
    define_variable::infect(interpreter)?;

    Ok(())
}
