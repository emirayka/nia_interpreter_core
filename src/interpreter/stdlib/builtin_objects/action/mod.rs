use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library::infect::infect_object_builtin_function;
use crate::interpreter::value::BuiltinFunctionType;

mod send_key_down;
mod send_key_press;
mod send_key_up;

mod send_mouse_button_down;
mod send_mouse_button_press;
mod send_mouse_button_up;

mod send_mouse_move_by;
mod send_mouse_move_to;

mod send_spawn;
mod send_text_type;

mod send_wait;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let action_object_id = interpreter.make_object();
    let action_symbol_id = interpreter.intern("action");

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec![
        ("send-key-down", send_key_down::send_key_down),
        ("send-key-press", send_key_press::send_key_press),
        ("send-key-up", send_key_up::send_key_up),
        (
            "send-mouse-button-down",
            send_mouse_button_down::send_mouse_button_down,
        ),
        (
            "send-mouse-button-press",
            send_mouse_button_press::send_mouse_button_press,
        ),
        (
            "send-mouse-button-up",
            send_mouse_button_up::send_mouse_button_up,
        ),
        ("send-mouse-move-by", send_mouse_move_by::send_mouse_move_by),
        ("send-mouse-move-to", send_mouse_move_to::send_mouse_move_to),
        ("send-text-type", send_text_type::send_text_type),
        ("send-spawn", send_spawn::send_spawn),
        ("send-wait", send_wait::send_wait),
    ];

    for (name, func) in bindings {
        infect_object_builtin_function(interpreter, action_object_id, name, func)?;
    }

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        action_symbol_id,
        Value::Object(action_object_id),
    )?;

    Ok(())
}
