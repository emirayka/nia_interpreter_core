use crate::BuiltinFunctionType;
use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

mod define_property_mark;
mod delete_property_mark;
mod freeze_mark;
mod get;
mod get_property_descriptor;
mod get_proto;
mod is_configurable_question;
mod is_enumerable_question;
mod is_frozen_question;
mod is_internable_question;
mod is_writable_question;
mod make;
mod new;
mod set_configurable_mark;
mod set_enumerable_mark;
mod set_internable_mark;
mod set_mark;
mod set_proto_mark;
mod set_writable_mark;
mod update_property_mark;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let object_object_id = interpreter.make_object();

    let funcs: Vec<(&str, BuiltinFunctionType)> = vec![
        (
            "define-property!",
            define_property_mark::define_property_mark,
        ),
        (
            "delete-property!",
            delete_property_mark::delete_property_mark,
        ),
        ("freeze!", freeze_mark::freeze_mark),
        ("make", make::make),
        ("get", get::get),
        (
            "get-property-descriptor",
            get_property_descriptor::get_property_descriptor,
        ),
        ("get-proto", get_proto::get_proto),
        (
            "is-configurable?",
            is_configurable_question::is_configurable_question,
        ),
        (
            "is-enumerable?",
            is_enumerable_question::is_enumerable_question,
        ),
        ("is-frozen?", is_frozen_question::is_frozen_question),
        (
            "is-internable?",
            is_internable_question::is_internable_question,
        ),
        ("is-writable?", is_writable_question::is_writable_question),
        (
            "set-configurable!",
            set_configurable_mark::set_configurable_mark,
        ),
        ("set-enumerable!", set_enumerable_mark::set_enumerable_mark),
        ("set-internable!", set_internable_mark::set_internable_mark),
        ("set!", set_mark::set_mark),
        ("set-proto!", set_proto_mark::set_proto_mark),
        ("set-writable!", set_writable_mark::set_writable_mark),
        ("new", new::new),
        (
            "update-property!",
            update_property_mark::update_property_mark,
        ),
    ];

    for (name, func) in funcs {
        library::infect_object_builtin_function(
            interpreter,
            object_object_id,
            name,
            func,
        )?;
    }

    let object_symbol_id = interpreter.intern_symbol_id("object");

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        object_symbol_id,
        Value::Object(object_object_id),
    )?;

    Ok(())
}
