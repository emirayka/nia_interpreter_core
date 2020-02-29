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
mod head;
mod head_while;
mod join;
mod last;
mod length;
mod map;
mod none_question;
mod nth;
mod reduce;
mod remove;
mod repeat;
mod replace;
mod tail;
mod tail_while;
mod unzip;
mod zip;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let list_object_id = interpreter.make_object();
    let list_symbol_id = interpreter.intern("list");

    let pairs: Vec<(&str, BuiltinFunctionType)> = vec!(
        ("all?", all_question::stub),
        ("any?", any_question::stub),
        ("butfirst", butfirst::stub),
        ("butlast", butlast::stub),
        ("contains?", contains_question::stub),
        ("filter", filter::stub),
        ("first", first::stub),
        ("head", head::stub),
        ("head-while", head_while::stub),
        ("join", join::stub),
        ("last", last::stub),
        ("length", length::stub),
        ("map", map::stub),
        ("none?", none_question::stub),
        ("nth", nth::stub),
        ("reduce", reduce::stub),
        ("remove", remove::stub),
        ("repeat", repeat::stub),
        ("replace", replace::stub),
        ("tail", tail::stub),
        ("tail-while", tail_while::stub),
        ("unzip", unzip::stub),
        ("zip", zip::stub)
    );

    for (name, func) in pairs {
        infect_object_builtin_function(
            interpreter,
            list_object_id,
            name,
            func
        );
    }

    interpreter.define_variable(
        interpreter.get_root_environment(),
        list_symbol_id,
        Value::Object(list_object_id)
    )?;

    Ok(())
}
