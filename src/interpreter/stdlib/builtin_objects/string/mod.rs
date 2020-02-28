use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::lib::infect::infect_object_builtin_function;
use crate::interpreter::function::builtin_function::BuiltinFunctionType;

mod compare;
mod concat;
mod contains;
mod equal_question;
mod find;
mod format;
mod greater_question;
mod join;
mod left;
mod length;
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

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec!(
        ("compare", compare::compare),
        ("concat", concat::concat),
        ("contains?", contains::contains),
        ("equal?", equal_question::equal_question),
        ("find", find::find),
        ("format", format::format),
        ("greater?", greater_question::greater_question),
        ("join", join::join),
        ("left", left::left),
        ("length", length::length),
        ("less?", less_question::less_question),
        ("lower", lower::lower),
        ("pad", pad::pad),
        ("pad-left", pad_left::pad_left),
        ("pad-right", pad_right::pad_right),
        ("repeat", repeat::repeat),
        ("right", right::right),
        ("split", split::split),
        ("trim", trim::trim),
        ("trim-left", trim_left::trim_left),
        ("trim-right", trim_right::trim_right),
        ("truncate", truncate::truncate),
        ("upper", upper::upper),
    );

    for (name, func) in bindings {
        infect_object_builtin_function(
            interpreter,
            string_object_id,
            name,
            func
        )?;
    }

    interpreter.define_variable(
        interpreter.get_root_environment(),
        string_symbol_id,
        Value::Object(string_object_id)
    )?;

    Ok(())
}
