use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::BuiltinFunctionType;
use crate::interpreter::value::Value;

use crate::library;

mod boolean_question;
mod cons_question;
mod float_question;
mod function_question;
mod int_question;
mod keyword_question;
mod object_question;
mod string_question;
mod symbol_question;

mod atom_question;
mod builtin_question;
mod even_question;
mod false_question;
mod interpreted_question;
mod list_question;
mod macro_question;
mod negative_question;
mod nil_question;
mod number_question;
mod odd_question;
mod positive_question;
mod special_question;
mod true_question;
mod zero_question;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let is_object_id = interpreter.make_object();

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec![
        ("int?", int_question::int_question),
        ("float?", float_question::float_question),
        ("boolean?", boolean_question::boolean_question),
        ("string?", string_question::string_question),
        ("symbol?", symbol_question::symbol_question),
        ("keyword?", keyword_question::keyword_question),
        ("cons?", cons_question::cons_question),
        ("object?", object_question::object_question),
        ("function?", function_question::function_question),
        ("false?", false_question::false_question),
        ("true?", true_question::true_question),
        ("nil?", nil_question::nil_question),
        ("number?", number_question::number_question),
        ("even?", even_question::even_question),
        ("odd?", odd_question::odd_question),
        ("negative?", negative_question::negative_question),
        ("zero?", zero_question::zero_question),
        ("positive?", positive_question::positive_question),
        ("list?", list_question::list_question),
        ("atom?", atom_question::atom_question),
        ("builtin?", builtin_question::builtin_question),
        ("interpreted?", interpreted_question::interpreted_question),
        ("macro?", macro_question::macro_question),
        ("special?", special_question::special_question),
    ];

    for (name, func) in bindings {
        library::infect_object_builtin_function(
            interpreter,
            is_object_id,
            name,
            func,
        )?;
    }

    let is_symbol_id = interpreter.intern_symbol_id("is");

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        is_symbol_id,
        Value::Object(is_object_id),
    )?;

    Ok(())
}
