use crate::interpreter::symbol::SymbolId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

use super::check_value_is_list;

pub fn add_value_to_root_list(
    interpreter: &mut Interpreter,
    name: &str,
    value: Value
) -> Result<(), Error> {
    let root_environment = interpreter.get_root_environment();
    let symbol_name = interpreter.intern(name);

    let root_variable = interpreter.lookup_variable(
        root_environment,
        symbol_name
    )?;

    check_value_is_list(interpreter, root_variable)?;

    let new_cons = interpreter.make_cons_value(
        value,
        root_variable
    );

    interpreter.set_variable(
        root_environment,
        symbol_name,
        new_cons
    )?;

    Ok(())
}

// todo: tests
