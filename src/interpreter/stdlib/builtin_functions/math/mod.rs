use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::lib::infect::infect_builtin_function;

mod sum;
mod sub;
mod mul;
mod div;
mod rem;
mod pow;

mod floor;
mod round;
mod ceiling;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_builtin_function(interpreter, "+", sum::sum)?;
    infect_builtin_function(interpreter, "-", sub::sub)?;
    infect_builtin_function(interpreter, "*", mul::mul)?;
    infect_builtin_function(interpreter, "/", div::div)?;
    infect_builtin_function(interpreter, "%", rem::rem)?;
    infect_builtin_function(interpreter, "pow",pow::pow)?;

    infect_builtin_function(interpreter, "floor",floor::floor)?;
    infect_builtin_function(interpreter, "round",round::round)?;
    infect_builtin_function(interpreter, "ceiling",ceiling::ceiling)?;

    Ok(())
}
