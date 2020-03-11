use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod func;
mod is;
mod keyboard;
mod list;
mod logic;
mod math;
mod object;
mod string;
mod to;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    func::infect(interpreter)?;
    is::infect(interpreter)?;
    keyboard::infect(interpreter)?;
    list::infect(interpreter)?;
    logic::infect(interpreter)?;
    math::infect(interpreter)?;
    object::infect(interpreter)?;
    string::infect(interpreter)?;
    to::infect(interpreter)?;

    Ok(())
}
