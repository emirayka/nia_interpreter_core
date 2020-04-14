use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod bit;
mod func;
mod is;
mod keyboard;
mod list;
mod logic;
mod math;
mod object;
mod rand;
mod string;
mod to;

mod nia; // :3

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    bit::infect(interpreter)?;
    func::infect(interpreter)?;
    is::infect(interpreter)?;
    keyboard::infect(interpreter)?;
    list::infect(interpreter)?;
    logic::infect(interpreter)?;
    math::infect(interpreter)?;
    object::infect(interpreter)?;
    rand::infect(interpreter)?;
    string::infect(interpreter)?;
    to::infect(interpreter)?;

    nia::infect(interpreter)?;

    Ok(())
}
