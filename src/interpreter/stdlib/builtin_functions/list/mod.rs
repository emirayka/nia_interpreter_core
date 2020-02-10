use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::lib::infect::infect_builtin_function;

mod list;
mod cons;
mod car;
mod cdr;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_builtin_function(interpreter, "list", list::list)?;
    infect_builtin_function(interpreter, "cons", cons::cons)?;
    infect_builtin_function(interpreter, "car", car::car)?;
    infect_builtin_function(interpreter, "cdr", cdr::cdr)?;

    Ok(())
}
