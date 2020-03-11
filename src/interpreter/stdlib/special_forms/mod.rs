use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library::infect::infect_special_form;
use crate::interpreter::function::SpecialFormFunctionType;

mod and;
mod cond;
mod quote;
mod define_variable;
mod define_function;
mod dolist;
mod function;
mod set;
mod fset;
mod _let;
mod let_star;
mod flet;
mod flet_star;
mod _match;
mod mlet;
mod mlet_star;
mod or;
mod progn;
mod block;
mod throw;
mod _try;
mod _while;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let pairs: Vec<(&str, SpecialFormFunctionType)> = vec!(
        ("and", and::and),
        ("cond", cond::cond),
        ("quote", quote::quote),
        ("define-variable", define_variable::define_variable),
        ("define-function", define_function::define_function),
        ("dolist", dolist::dolist),
        ("function", function::function),
        ("fset!", fset:: fset),
        ("let", _let::_let),
        ("let*", let_star::let_star),
        ("flet", flet::flet),
        ("flet*", flet_star::flet_star),
        ("match", _match::_match),
        ("mlet", mlet::mlet),
        ("mlet*", mlet_star::mlet_star),
        ("or", or::or),
        ("progn", progn::progn),
        ("set!", set::set),
        ("block", block::block),
        ("throw", throw::throw),
        ("try", _try::_try),
        ("while", _while::_while),
    );

    for (name, func) in pairs {
        infect_special_form(
            interpreter,
            name,
            func
        )?;
    }

    Ok(())
}
