use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::SpecialFormFunctionType;

use crate::library;

mod _let;
mod _match;
mod _try;
mod _while;
mod and;
mod block;
mod call_with_this;
mod cond;
mod define_function;
mod define_variable;
mod dolist;
mod dotimes;
mod export;
mod flet;
mod flet_star;
mod fset;
mod function;
mod import;
mod let_star;
mod mlet;
mod mlet_star;
mod or;
mod progn;
mod quote;
mod set;
mod throw;
mod with_this;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let pairs: Vec<(&str, SpecialFormFunctionType)> = vec![
        ("and", and::and),
        ("call-with-this", call_with_this::call_with_this),
        ("cond", cond::cond),
        ("quote", quote::quote),
        ("define-variable", define_variable::define_variable),
        ("define-function", define_function::define_function),
        ("dolist", dolist::dolist),
        ("dotimes", dotimes::dotimes),
        ("export", export::export),
        ("function", function::function),
        ("fset!", fset::fset),
        ("import", import::import),
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
        ("with-this", with_this::with_this),
    ];

    for (name, func) in pairs {
        library::infect_special_form(interpreter, name, func)?;
    }

    Ok(())
}
