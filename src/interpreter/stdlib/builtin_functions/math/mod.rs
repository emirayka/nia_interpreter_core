use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::lib::infect::infect_builtin_function;

mod sum;
mod sub;
mod mul;
mod div;
mod rem;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_builtin_function(interpreter, "+", sum::sum)?;
    infect_builtin_function(interpreter, "-", sub::sub)?;
    infect_builtin_function(interpreter, "*", mul::mul)?;
    infect_builtin_function(interpreter, "/", div::div)?;
    infect_builtin_function(interpreter, "%", rem::rem)?;

    Ok(())
}
