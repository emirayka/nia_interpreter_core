use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::BuiltinFunctionType;
use crate::interpreter::value::Value;

use crate::library;

mod all_question;
mod any_question;
mod aperture;
mod append;
mod contains_question;
mod filter;
mod fold;
mod foldl;
mod head;
mod init;
mod join;
mod last;
mod length;
mod map;
mod new;
mod none_question;
mod nth;
mod remove;
mod repeat;
mod replace;
mod reverse;
mod set_nth_mark;
mod tail;
mod take;
mod take_while;
mod unzip;
mod zip;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let list_object_id = interpreter.make_object();
    let list_symbol_id = interpreter.intern_symbol_id("list");

    let pairs: Vec<(&str, BuiltinFunctionType)> = vec![
        ("all?", all_question::all_question),
        ("any?", any_question::any_question),
        ("append", append::append),
        ("aperture", aperture::aperture),
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
        ("new", new::new),
        ("none?", none_question::none_question),
        ("nth", nth::nth),
        ("remove", remove::remove),
        ("repeat", repeat::repeat),
        ("replace", replace::replace),
        ("reverse", reverse::reverse),
        ("set-nth!", set_nth_mark::set_nth_mark),
        ("tail", tail::tail),
        ("take", take::take),
        ("take-while", take_while::take_while),
        ("unzip", unzip::unzip),
        ("zip", zip::zip),
    ];

    for (name, func) in pairs {
        library::infect_object_builtin_function(
            interpreter,
            list_object_id,
            name,
            func,
        )?;
    }

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        list_symbol_id,
        Value::Object(list_object_id),
    )?;

    Ok(())
}
