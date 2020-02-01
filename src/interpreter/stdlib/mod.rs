use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::error::Error;

mod special_forms;

pub fn infect_interpreter(interpreter: &mut Interpreter) -> Result<(), Error> {
    special_forms::infect_special_forms(interpreter)?;

    Ok(())
}