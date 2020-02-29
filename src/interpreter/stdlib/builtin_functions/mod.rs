use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::function::builtin_function::BuiltinFunctionType;
use crate::interpreter::lib::infect::infect_builtin_function;

mod car;
mod cdr;
mod cons;
mod div;
mod flookup;
mod gensym;
mod intern;
mod list;
mod lookup;
mod mul;
mod rem;
mod sub;
mod sum;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let pairs: Vec<(&str, BuiltinFunctionType)> = vec!(
        ("car", car::car),
        ("cdr", cdr::cdr),
        ("cons", cons::cons),
        ("list", list::list),

        ("flookup", flookup::flookup),
        ("gensym", gensym::gensym),
        ("intern", intern::intern),
        ("lookup", lookup::lookup),

        ("/", div::div),
        ("*", mul::mul),
        ("%", rem::rem),
        ("-", sub::sub),
        ("+", sum::sum),
    );

    for (name, func) in pairs {
        infect_builtin_function(interpreter, name, func)?;
    }

    Ok(())
}
