use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library::infect::infect_object_builtin_function;
use crate::interpreter::function::BuiltinFunctionType;

mod all_question;
mod any_question;
mod append;
mod contains_question;
mod filter;
mod foldl;
mod fold;
mod head;
mod init;
mod join;
mod last;
mod length;
mod map;
mod none_question;
mod nth;
mod remove;
mod repeat;
mod replace;
mod reverse;
mod tail;
mod take;
mod take_while;
mod unzip;
mod zip;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let list_object_id = interpreter.make_object();
    let list_symbol_id = interpreter.intern("list");

    let pairs: Vec<(&str, BuiltinFunctionType)> = vec!(
        ("all?", all_question::all_question),
        ("any?", any_question::any_question),
        ("append", append::append),
        ("contains?", contains_question::contains),
        ("filter", filter::filter),
        ("fold", fold::fold),
        ("foldl", foldl::foldl),
        ("head", head::head),
        ("init", init::init),
        ("join", join::join),
        ("last", last::last),
        ("length", length::length),
        ("map", map::map),
        ("none?", none_question::none_question),
        ("nth", nth::nth),
        ("remove", remove::remove),
        ("repeat", repeat::repeat),
        ("replace", replace::replace),
        ("reverse", reverse::reverse),
        ("tail", tail::tail),
        ("take", take::take),
        ("take-while", take_while::take_while),
        ("unzip", unzip::unzip),
        ("zip", zip::zip)
    );

    for (name, func) in pairs {
        infect_object_builtin_function(
            interpreter,
            list_object_id,
            name,
            func
        )?;
    }

    interpreter.define_variable(
        interpreter.get_root_environment(),
        list_symbol_id,
        Value::Object(list_object_id)
    )?;

    Ok(())
}
