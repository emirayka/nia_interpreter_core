use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod action;
mod bit;
mod cons;
mod device;
mod func;
mod is;
mod list;
mod logic;
mod math;
mod object;
mod rand;
mod string;
mod to;

mod nia; // :3

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    action::infect(interpreter)?;
    bit::infect(interpreter)?;
    cons::infect(interpreter)?;
    func::infect(interpreter)?;
    is::infect(interpreter)?;
    device::infect(interpreter)?;
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
