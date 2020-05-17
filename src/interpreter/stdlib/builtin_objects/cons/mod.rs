use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::BuiltinFunctionType;
use crate::interpreter::value::Value;

use crate::library;

mod car;
mod cdr;
mod new;
mod set_car_mark;
mod set_cdr_mark;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let cons_object_id = interpreter.make_object();
    let cons_symbol_id = interpreter.intern_symbol_id("cons");

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec![
        ("new", new::new),
        ("car", car::car),
        ("cdr", cdr::cdr),
        ("set-car!", set_car_mark::set_car_mark),
        ("set-cdr!", set_cdr_mark::set_cdr_mark),
    ];

    for (name, func) in bindings {
        library::infect_object_builtin_function(
            interpreter,
            cons_object_id,
            name,
            func,
        )?;
    }

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        cons_symbol_id,
        Value::Object(cons_object_id),
    )?;

    Ok(())
}
