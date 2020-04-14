use std::fs::File;
use std::thread;
use std::sync::mpsc;

use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::keyboard_remapper::{
    KeyId,
    KeyboardId,
    KeyChord,
    KeyboardListener,
    KeyboardListenerUnifier,
    KeyChordProducer,
};
use crate::interpreter::library;


pub fn start_event_loop(
    interpreter: &mut Interpreter,
    keyboards: Vec<(String, String)>
) -> Result<(), Error>{
    let mut keyboard_listener_unifier = KeyboardListenerUnifier::new();

    for (index, keyboard) in keyboards.into_iter().enumerate() {
        let keyboard_listener = KeyboardListener::new(
            KeyboardId::new(index),
            String::from(keyboard.0)
        );

        keyboard_listener_unifier.add_keyboard_listener(keyboard_listener);
    }

    let key_chord_producer = KeyChordProducer::new(
        keyboard_listener_unifier,
        vec!()
    );

    let receiver = key_chord_producer.start_listening();

    loop {
        let key_chord = receiver.recv().unwrap();

        println!("{:?}", key_chord);
    }

    Ok(())
}

pub fn start_loop(interpreter: &mut Interpreter) -> Result<(), Error> {
    let root_environment_id = interpreter.get_root_environment();
    let symbol_id_registered_keyboards = interpreter.intern("registered-keyboards");

    let registered_keyboards = interpreter.lookup_variable(
        root_environment_id,
        symbol_id_registered_keyboards,
    )?;

    library::check_value_is_cons(
        interpreter,
        registered_keyboards,
    )?;

    let registered_keyboards = interpreter.list_to_vec(
        registered_keyboards.as_cons_id()
    )?;

    let mut keyboards = Vec::new();

    for registered_keyboard in registered_keyboards {
        library::check_value_is_cons(
            interpreter,
            registered_keyboard,
        )?;

        let registered_keyboard = interpreter.list_to_vec(
            registered_keyboard.as_cons_id()
        )?;

        let path = library::read_as_string(interpreter, registered_keyboard[0])?;
        let name = library::read_as_string(interpreter, registered_keyboard[1])?;

        keyboards.push((path.clone(), name.clone()))
    }

    start_event_loop(
        interpreter,
        keyboards
    )?;

    Ok(())
}

pub fn start_listening(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 0 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `keyboard:start-listening' takes no arguments."
        ).into_result();
    }

    start_loop(interpreter)?;

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {}
