use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::lib::infect::infect_object_builtin_function;

mod compare;
mod concat;
mod contains;
mod count;
mod equal_question;
mod find;
mod format;
mod greater_question;
mod join;
mod left;
mod less_question;
mod lower;
mod pad;
mod pad_left;
mod pad_right;
mod repeat;
mod right;
mod split;
mod trim;
mod trim_left;
mod trim_right;
mod truncate;
mod upper;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let string_object_id = interpreter.make_object();
    let string_symbol_id = interpreter.intern("string");

    infect_object_builtin_function(
        interpreter,
        string_object_id,
        "compare",
        compare::compare
    )?;

    interpreter.define_variable(
        interpreter.get_root_environment(),
        string_symbol_id,
        Value::Object(string_object_id)
    )?;

    Ok(())
}