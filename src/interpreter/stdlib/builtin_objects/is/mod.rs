use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library::infect::{infect_object_builtin_function};
use crate::interpreter::value::Value;
use crate::interpreter::value::BuiltinFunctionType;

pub mod int_question;
pub mod float_question;
pub mod boolean_question;
pub mod string_question;
pub mod symbol_question;
pub mod keyword_question;
pub mod cons_question;
pub mod object_question;
pub mod function_question;

pub mod false_question;
pub mod true_question;
pub mod nil_question;
pub mod number_question;
pub mod even_question;
pub mod odd_question;
pub mod negative_question;
pub mod zero_question;
pub mod positive_question;
pub mod list_question;
pub mod atom_question;
pub mod builtin_question;
pub mod interpreted_question;
pub mod macro_question;
pub mod special_question;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let is_object_id = interpreter.make_object();

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec!(
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
    );

    for (name, func) in bindings {
        infect_object_builtin_function(
            interpreter,
            is_object_id,
            name,
            func
        )?;
    }

    let is_symbol_id = interpreter.intern("is");

    interpreter.define_variable(
        interpreter.get_root_environment(),
        is_symbol_id,
        Value::Object(is_object_id)
    )?;

    Ok(())
}
