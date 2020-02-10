use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::lib::infect::infect_builtin_function;

mod intern;
mod gensym;
mod lookup;
mod flookup;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_builtin_function(interpreter, "intern", intern::intern)?;
    infect_builtin_function(interpreter, "gensym",gensym::gensym)?;
    infect_builtin_function(interpreter, "lookup",lookup::lookup)?;
    infect_builtin_function(interpreter, "flookup",flookup::flookup)?;

    Ok(())
}
