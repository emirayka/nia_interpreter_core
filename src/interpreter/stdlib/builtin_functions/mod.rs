use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::function::BuiltinFunctionType;
use crate::interpreter::library::infect::infect_builtin_function;

mod assert;
mod car;
mod cdr;
mod cons;
mod div;
mod eq_question;
mod equal_question;
mod eval;
mod flookup;
mod gensym;
mod intern;
mod list;
mod lookup;
mod mul;
mod not;
mod rem;
mod sub;
mod sum;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let pairs: Vec<(&str, BuiltinFunctionType)> = vec!(
        ("assert", assert::assert),
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

        ("not", not::not),
        ("eq?", eq_question::eq_question),
        ("=", eq_question::eq_question),
        ("equal?", equal_question::equal_question),

        ("eval", eval::eval)
    );

    for (name, func) in pairs {
        infect_builtin_function(interpreter, name, func)?;
    }

    Ok(())
}
