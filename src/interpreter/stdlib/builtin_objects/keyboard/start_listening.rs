use std::collections::HashMap;

use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::library;

use nia_events::{KeyChord, KeyChordPart, KeyId, KeyboardId};
use nia_state_machine;
use nom::combinator::map;

fn read_keyboards(
    interpreter: &mut Interpreter
) -> Result<Vec<(String, String)>, Error> {
    let registered_keyboards = library::get_root_variable(
        interpreter,
        "registered-keyboards"
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

    Ok(keyboards)
}

fn read_modifiers(
    interpreter: &mut Interpreter
) -> Result<Vec<KeyChordPart>, Error> {
    let modifiers_value = library::get_root_variable(
        interpreter,
        "modifiers"
    )?;

    library::check_value_is_list(interpreter, modifiers_value);

    let modifiers_values = interpreter.list_to_vec(modifiers_value.as_cons_id())?;
    let mut modifiers = Vec::new();

    for modifier_value in modifiers_values {
        let modifier = library::read_as_key_chord_part(
            interpreter,
            modifier_value
        )?;

        modifiers.push(modifier);
    }

    Ok(modifiers)
}

fn read_key_chords(
    interpreter: &mut Interpreter,
    key_chords_value: Value
) -> Result<Vec<KeyChord>, Error> {
    library::check_value_is_cons(interpreter, key_chords_value)?;

    let key_chords_values = interpreter.list_to_vec(key_chords_value.as_cons_id())?;
    let mut key_chords = Vec::new();

    for key_chord_value in key_chords_values {
        let key_chord = library::read_as_key_chord(interpreter, key_chord_value)?;

        key_chords.push(key_chord);
    }

    Ok(key_chords)
}

fn read_mapping(
    interpreter: &mut Interpreter,
    mapping_value: Value
) -> Result<(Vec<KeyChord>, Value), Error> {
    library::check_value_is_cons(interpreter, mapping_value)?;

    let values = interpreter.list_to_vec(mapping_value.as_cons_id())?;
    let key_chords = read_key_chords(interpreter, values[0])?;
    let function = values[1];

    Ok((key_chords, function))
}

fn read_mappings(
    interpreter: &mut Interpreter
) -> Result<Vec<(Vec<KeyChord>, Value)>, Error> {
    let mappings = library::get_root_variable(
        interpreter,
        "global-map"
    )?;

    library::check_value_is_cons(interpreter, mappings)?;

    let mappings_values = interpreter.list_to_vec(mappings.as_cons_id())?;
    let mut mappings = Vec::new();

    for mapping_value in mappings_values {
        let mapping = read_mapping(interpreter, mapping_value)?;
        mappings.push(mapping);
    }

    Ok(mappings)
}

fn start_event_loop(
    interpreter: &mut Interpreter,
    keyboards: Vec<(String, String)>,
    modifiers: Vec<KeyChordPart>,
    mappings: Vec<(Vec<KeyChord>, Value)>
) -> Result<(), Error> {
    let mut settings_builder = nia_events::EventListenerSettingsBuilder::new();
    let mut map = HashMap::new();

    for (index, (keyboard_path, keyboard_name)) in keyboards.into_iter().enumerate() {
        settings_builder = settings_builder.add_keyboard(keyboard_path);
        map.insert(keyboard_name, nia_events::KeyboardId::new(index as u16));
    }

    for modifier in modifiers {
        settings_builder = settings_builder.add_modifier(modifier);
    }

    let settings = settings_builder.build();

    let event_listener = nia_events::EventListener::new(settings);
    let receiver = event_listener.start_listening();

    let mut state_machine = nia_state_machine::StateMachine::new();

    for (path, action) in mappings {
        state_machine.add(path, action)
            .map_err(|_| interpreter.make_invalid_argument_error(
                "Can't bind binding."
            ));
    }

    loop {
        let event = receiver.recv()
            .expect("Failure while listening event.");
        
        println!("{:?}", event);

        match event {
            nia_events::Event::KeyChordEvent(key_chord) => {
                match state_machine.excite(&key_chord) {
                    Some(value) => {
                        let root_environment_id = interpreter.get_root_environment();
                        let nil = interpreter.intern_nil_symbol_value();
                        let value_to_execute = interpreter.make_cons_value(
                            *value,
                            nil
                        );

                        interpreter.evaluate_value(
                            root_environment_id,
                            value_to_execute
                        ).expect("");
                    },
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

pub fn start_loop(
    interpreter: &mut Interpreter,
) -> Result<(), Error> {
    let keyboards = read_keyboards(interpreter)?;
    let modifiers = read_modifiers(interpreter)?;
    let mappings = read_mappings(interpreter)?;

    start_event_loop(
        interpreter,
        keyboards,
        modifiers,
        mappings
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
