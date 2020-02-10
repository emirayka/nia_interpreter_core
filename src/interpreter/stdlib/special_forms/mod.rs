use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::lib::infect::infect_special_form;

mod _lib;

mod cond;
mod quote;
mod define_variable;
mod define_function;
mod function;
mod set;
mod fset;
mod _let;
mod let_star;
mod flet;
mod flet_star;
mod mlet;
mod mlet_star;
mod progn;
mod block;
mod throw;
mod _try;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_special_form(interpreter, "cond", cond::cond)?;
    infect_special_form(interpreter, "quote", quote::quote)?;
    infect_special_form(interpreter, "define-variable", define_variable::define_variable)?;
    infect_special_form(interpreter, "define-function", define_function::define_function)?;
    infect_special_form(interpreter, "function", function::function)?;
    infect_special_form(interpreter, "set!", set::set)?;
    infect_special_form(interpreter, "fset!", fset:: fset)?;
    infect_special_form(interpreter, "let", _let::_let)?;
    infect_special_form(interpreter, "let*", let_star::let_star)?;
    infect_special_form(interpreter, "flet", flet::flet)?;
    infect_special_form(interpreter, "flet*", flet_star::flet_star)?;
    infect_special_form(interpreter, "mlet", mlet::mlet)?;
    infect_special_form(interpreter, "mlet*", mlet_star::mlet_star)?;
    infect_special_form(interpreter, "progn", progn::progn)?;
    infect_special_form(interpreter, "block", block::block)?;
    infect_special_form(interpreter, "throw", throw::throw)?;
    infect_special_form(interpreter, "try", _try::_try)?;

    Ok(())
}
