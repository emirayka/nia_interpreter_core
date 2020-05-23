use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::BuiltinFunctionType;
use crate::interpreter::value::Value;

use crate::library;

mod send_key_click;
mod send_key_press;
mod send_key_release;

mod send_mouse_button_click;
mod send_mouse_button_press;
mod send_mouse_button_release;

mod send_mouse_absolute_move;
mod send_mouse_relative_move;

mod send_execute_os_command;
mod send_text_type;

mod send_wait;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let action_object_id = interpreter.make_object();
    let action_symbol_id = interpreter.intern_symbol_id("action");

    let bindings: Vec<(&str, BuiltinFunctionType)> = vec![
        ("send-key-press", send_key_press::send_key_press),
        ("send-key-click", send_key_click::send_key_click),
        ("send-key-release", send_key_release::send_key_release),
        (
            "send-mouse-button-press",
            send_mouse_button_press::send_mouse_button_press,
        ),
        (
            "send-mouse-button-click",
            send_mouse_button_click::send_mouse_button_click,
        ),
        (
            "send-mouse-button-release",
            send_mouse_button_release::send_mouse_button_release,
        ),
        (
            "send-mouse-relative-move",
            send_mouse_relative_move::send_mouse_relative_move,
        ),
        (
            "send-mouse-absolute-move",
            send_mouse_absolute_move::send_mouse_absolute_move,
        ),
        ("send-text-type", send_text_type::send_text_type),
        (
            "send-execute-os-command",
            send_execute_os_command::send_execute_os_command,
        ),
        ("send-wait", send_wait::send_wait),
    ];

    for (name, func) in bindings {
        library::infect_object_builtin_function(
            interpreter,
            action_object_id,
            name,
            func,
        )?;
    }

    interpreter.define_variable(
        interpreter.get_root_environment_id(),
        action_symbol_id,
        Value::Object(action_object_id),
    )?;

    Ok(())
}
