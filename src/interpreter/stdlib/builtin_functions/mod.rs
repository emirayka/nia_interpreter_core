use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::BuiltinFunctionType;

use crate::library;

mod _println;
mod _type;
mod assert;
mod dec;
mod div;
mod eq_question;
mod equal_question;
mod eval;
mod flookup;
mod ge;
mod gensym;
mod gt;
mod inc;
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
mod string;
mod sub;
mod sum;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let pairs: Vec<(&str, BuiltinFunctionType)> = vec![
        ("assert", assert::assert),
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
        ("println", _println::_println),
    ];

    for (name, func) in pairs {
        library::infect_builtin_function(interpreter, name, func)?;
    }

    Ok(())
}
