use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::error::Error;

mod special_forms;
mod builtin_functions;
mod builtin_objects;

mod core;

pub fn infect_stdlib(interpreter: &mut Interpreter) -> Result<(), Error> {
    special_forms::infect(interpreter)?;
    builtin_functions::infect(interpreter)?;
    builtin_objects::infect(interpreter)?;

    core::infect(interpreter)?;

    Ok(())
}