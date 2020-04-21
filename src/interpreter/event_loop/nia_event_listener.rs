use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::sync::mpsc::TryRecvError;

use nia_events::{KeyChordPart, EventListener};
use nia_events::Event;
use nia_events::KeyCommand;
use nia_events::EventListenerSettingsBuilder;
use nia_events::KeyboardId;
use nia_events::Command;
use nia_events::KeyChord;

use nia_state_machine::StateMachineResult;

use crate::interpreter::{Interpreter, Value, Action, Error};

use crate::interpreter::library;
use std::convert::TryFrom;

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

    library::check_value_is_list(interpreter, modifiers_value)?;

    let modifiers_values = library::read_as_vector(
        interpreter,
        modifiers_value
    )?;
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

    library::check_value_is_list(interpreter, mappings)?;

    let mappings_values = library::read_as_vector(
        interpreter,
        mappings
    )?;
    let mut mappings = Vec::new();

    for mapping_value in mappings_values {
        let mapping = read_mapping(interpreter, mapping_value)?;
        mappings.push(mapping);
    }

    Ok(mappings)
}


pub struct NiaEventListener {
    keyboards: Vec<(String, String)>,
    modifiers: Vec<KeyChordPart>,
    mappings: Vec<(Vec<KeyChord>, Action)>
}

impl NiaEventListener {
    pub fn new() -> NiaEventListener {
        NiaEventListener {
            keyboards: Vec::new(),
            modifiers: Vec::new(),
            mappings: Vec::new(),
        }
    }

    pub fn from_interpreter(interpreter: &mut Interpreter) -> NiaEventListener {
        let mut event_listener = NiaEventListener::new();

        let keyboards = read_keyboards(interpreter)
            .expect("Failed keyboard reading.");

        let modifiers = read_modifiers(interpreter)
            .expect("Failed modifiers' reading.");

        let mappings = read_mappings(interpreter)
            .expect("Failed mappings' reading.");

        for keyboard in keyboards {
            event_listener.add_keyboard(&keyboard.0, &keyboard.1);
        }

        for modifier in modifiers {
            event_listener.add_modifier(modifier);
        }

        for mapping in mappings {
            let action = Action::from(mapping.1);

            event_listener.add_mapping(
                 mapping.0,
                action
            );
        }

        event_listener
    }

    pub fn add_keyboard(&mut self, path: &str, name: &str) {
        self.keyboards.push((String::from(path), String::from(name)))
    }

    pub fn add_modifier(&mut self, modifier: KeyChordPart) {
        self.modifiers.push(modifier)
    }

    pub fn add_mapping(&mut self, key_chords: Vec<KeyChord>, action: Action) {
        self.mappings.push((key_chords, action))
    }

    pub fn start_listening(
        &self
    ) -> Result<(mpsc::Sender<Command>, mpsc::Receiver<Action>, mpsc::Sender<()>), Error> {
        let mut settings_builder = EventListenerSettingsBuilder::new();
        let mut map = HashMap::new();
        let mut iterator = self.keyboards.iter().enumerate();

        for (index, (keyboard_path, keyboard_name)) in iterator {
            settings_builder = settings_builder.add_keyboard(keyboard_path.clone());
            map.insert(keyboard_name.clone(), KeyboardId::new(index as u16));
        }

        for modifier in self.modifiers.iter() {
            settings_builder = settings_builder.add_modifier(*modifier);
        }

        let settings = settings_builder.build();

        let event_listener = EventListener::new(settings);
        let (event_receiver, event_stopper) = event_listener.start_listening();

        let command_sender = nia_events::CommandSender::new();
        let (cmd_sender, cmd_stopper) = command_sender.start_sending();

        let mut state_machine = nia_state_machine::StateMachine::new();

        for (path, action) in self.mappings.iter() {
            state_machine.add(path.clone(), action.clone())
                .map_err(|_| Error::failure(String::from("")))?;
        }

        let (action_sender, action_receiver) = mpsc::channel();
        let (tx, rx) = mpsc::channel();

        {
            let action_sender = action_sender.clone();
            let cmd_sender = cmd_sender.clone();

            thread::spawn(move || {
                loop {
                    let event = match event_receiver.recv() {
                        Ok(event) => {
                            event
                        },
                        Err(_) => break
                    };

                    println!("{:?}", event);

                    match event {
                        Event::KeyChordEvent(key_chord) => {
                            match state_machine.excite(key_chord) {
                                StateMachineResult::Excited(action) => {
                                    action_sender.send(action);
                                },
                                StateMachineResult::Fallback(previous) => {
                                    for key_chord in previous {
                                        let command = nia_events::Command::KeyCommand(
                                            KeyCommand::ForwardKeyChord(key_chord)
                                        );

                                        match cmd_sender.send(command) {
                                            Ok(_) => {},
                                            Err(_) => break
                                        }
                                    }
                                },
                                StateMachineResult::Transition() => {}
                            }
                        }
                    }

                    match rx.try_recv() {
                        Ok(()) | Err(TryRecvError::Disconnected) => {
                            break;
                        },
                        Err(TryRecvError::Empty) => {}
                    }
                }

                cmd_stopper.send(());
                event_stopper.send(());
            });
        }

        Ok((cmd_sender, action_receiver, tx))
    }
}
