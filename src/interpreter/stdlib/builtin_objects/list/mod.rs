use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::lib::infect::infect_object_builtin_function;
use crate::interpreter::function::builtin_function::BuiltinFunctionType;

mod all_question;
mod any_question;
mod butfirst;
mod butlast;
mod contains_question;
mod filter;
mod first;
mod foldl;
mod fold;
mod head;
mod join;
mod last;
mod length;
mod map;
mod none_question;
mod nth;
mod remove;
mod repeat;
mod replace;
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
        ("any?", any_question::stub),
        ("butfirst", butfirst::stub),
        ("butlast", butlast::stub),
        ("contains?", contains_question::stub),
        ("filter", filter::stub),
        ("first", first::stub),
        ("fold", fold::fold),
        ("foldl", foldl::foldl),
        ("head", head::head),
        ("join", join::stub),
        ("last", last::stub),
        ("length", length::stub),
        ("map", map::stub),
        ("none?", none_question::stub),
        ("nth", nth::stub),
        ("remove", remove::stub),
        ("repeat", repeat::stub),
        ("replace", replace::stub),
        ("tail", tail::tail),
        ("take", take::take),
        ("take-while", take_while::take_while),
        ("unzip", unzip::stub),
        ("zip", zip::stub)
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
