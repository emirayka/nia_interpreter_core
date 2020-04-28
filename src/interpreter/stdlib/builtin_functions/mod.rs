use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::BuiltinFunctionType;
use crate::interpreter::library::infect::infect_builtin_function;

mod assert;
mod car;
mod cdr;
mod cons;
mod dec;
mod div;
mod eq_question;
mod equal_question;
mod eval;
mod inc;
mod flookup;
mod ge;
mod gensym;
mod gt;
mod intern;
mod le;
mod list;
mod lookup;
mod lt;
mod mul;
mod neq_question;
mod nequal_question;
mod not;
mod rem;
mod set_car_mark;
mod set_cdr_mark;
mod string;
mod sub;
mod sum;
mod _type;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let pairs: Vec<(&str, BuiltinFunctionType)> = vec!(
        ("assert", assert::assert),
        ("car", car::car),
        ("cdr", cdr::cdr),
        ("set-car!", set_car_mark::set_car_mark),
        ("set-cdr!", set_cdr_mark::set_cdr_mark),
        ("cons", cons::cons),
        ("list", list::list),
        ("string", string::string),

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
        ("equal?", equal_question::equal_question),
        ("neq?", neq_question::neq_question),
        ("nequal?", nequal_question::nequal_question),
        ("=", eq_question::eq_question),
        ("==", equal_question::equal_question),
        ("!=", neq_question::neq_question),
        ("!==", nequal_question::nequal_question),
        ("<", lt::lt),
        ("<=", le::le),
        (">", gt::gt),
        (">=", ge::ge),

        ("dec", dec::dec),
        ("inc", inc::inc),
        ("eval", eval::eval),
        ("type", _type::_type),
    );

    for (name, func) in pairs {
        infect_builtin_function(interpreter, name, func)?;
    }

    Ok(())
}
