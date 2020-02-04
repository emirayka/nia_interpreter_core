use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

mod _lib;

mod cond;
mod quote;
mod define_variable;
mod define_function;
mod function;
mod set;
mod _let;
mod let_star;
mod flet;
mod flet_star;
mod mlet;
mod mlet_star;
mod progn;

pub fn infect_special_forms(interpreter: &mut Interpreter) -> Result<(), Error> {
    cond::infect(interpreter)?;
    quote::infect(interpreter)?;
    define_variable::infect(interpreter)?;
    define_function::infect(interpreter)?;
    function::infect(interpreter)?;
    set::infect(interpreter)?;
    _let::infect(interpreter)?;
    let_star::infect(interpreter)?;
    flet::infect(interpreter)?;
    flet_star::infect(interpreter)?;
    mlet::infect(interpreter)?;
    mlet_star::infect(interpreter)?;
    progn::infect(interpreter)?;

    Ok(())
}
