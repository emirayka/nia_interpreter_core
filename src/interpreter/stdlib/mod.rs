use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::error::Error;

mod special_forms;

pub fn infect_interpreter(interpreter: &mut Interpreter) -> Result<(), Error> {
    special_forms::cond::infect(interpreter)?;
    special_forms::quote::infect(interpreter)?;

    Ok(())
}