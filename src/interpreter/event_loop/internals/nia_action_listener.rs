use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::sync::mpsc;
use std::thread;

use nia_events::Event;
use nia_events::KeyChord;
use nia_events::KeyChordPart;
use nia_events::KeyboardId;
use nia_events::Listener;
use nia_events::ListenerSettingsBuilder;
use nia_events::UInputWorkerCommand;
use nia_events::WorkerHandle;
use nia_events::{Command, ListenerSettings};

use nia_state_machine::{StateMachine, StateMachineResult};

use crate::Error;
use crate::EventLoopHandle;
use crate::Interpreter;
use crate::Value;
use crate::{Action, NiaActionListenerHandle};

use crate::library;
use std::time::Duration;

pub struct NiaActionListener {
    keyboards: Vec<(String, String)>,
    modifiers: Vec<KeyChordPart>,
    mappings: Vec<(Vec<KeyChord>, Action)>,
}

impl NiaActionListener {
    pub fn new() -> NiaActionListener {
        NiaActionListener {
            keyboards: Vec::new(),
            modifiers: Vec::new(),
            mappings: Vec::new(),
        }
    }

    pub fn from_interpreter(
        interpreter: &mut Interpreter,
    ) -> Result<NiaActionListener, Error> {
        let mut event_listener = NiaActionListener::new();

        let keyboards = library::read_keyboards(interpreter)?;
        let modifiers = library::read_modifiers(interpreter)?;
        let mappings = library::read_mappings(interpreter)?;

        for keyboard in keyboards {
            event_listener.add_keyboard(&keyboard.0, &keyboard.1);
        }

        for modifier in modifiers {
            event_listener.add_modifier(modifier);
        }

        for mapping in mappings {
            let action = Action::from(mapping.1);

            event_listener.add_mapping(mapping.0, action);
        }

        Ok(event_listener)
    }

    pub fn add_keyboard(&mut self, path: &str, name: &str) {
        self.keyboards
            .push((String::from(path), String::from(name)))
    }

    pub fn add_modifier(&mut self, modifier: KeyChordPart) {
        self.modifiers.push(modifier)
    }

    pub fn add_mapping(&mut self, key_chords: Vec<KeyChord>, action: Action) {
        self.mappings.push((key_chords, action))
    }

    fn build_settings(&self) -> ListenerSettings {
        let mut settings_builder = ListenerSettingsBuilder::new();
        let mut map = HashMap::new();
        let mut iterator = self.keyboards.iter().enumerate();

        for (index, (keyboard_path, keyboard_name)) in iterator {
            settings_builder =
                settings_builder.add_keyboard(keyboard_path.clone());
            map.insert(keyboard_name.clone(), KeyboardId::new(index as u16));
        }

        for modifier in self.modifiers.iter() {
            settings_builder = settings_builder.add_modifier(*modifier);
        }

        settings_builder.build()
    }

    fn construct_state_machine(
        &self,
    ) -> Result<StateMachine<KeyChord, Action>, Error> {
        let mut state_machine = nia_state_machine::StateMachine::new();

        for (path, action) in self.mappings.iter() {
            state_machine
                .add(path.clone(), action.clone())
                .map_err(|_| Error::failure(String::from("")))?;
        }

        Ok(state_machine)
    }

    pub fn start_listening(
        &self,
        worker_handle: WorkerHandle,
    ) -> Result<NiaActionListenerHandle, Error> {
        let listener_settings = self.build_settings();
        let listener = Listener::new(listener_settings);
        let listener_handle = listener.start_listening();

        let mut state_machine = self.construct_state_machine()?;

        let (action_sender, action_receiver) = mpsc::channel();
        let (stop_sender, stop_receiver) = mpsc::channel();

        {
            let action_sender = action_sender;
            let worker_handle = worker_handle;

            thread::spawn(move || {
                loop {
                    let event = match listener_handle.try_receive_event() {
                        Ok(event) => Some(event),
                        Err(mpsc::TryRecvError::Disconnected) => {
                            break;
                        }
                        Err(mpsc::TryRecvError::Empty) => None,
                    };

                    match event {
                        Some(Event::KeyChordEvent(key_chord)) => {
                            match state_machine.excite(key_chord) {
                                StateMachineResult::Excited(action) => {
                                    action_sender.send(action);
                                }
                                StateMachineResult::Fallback(previous) => {
                                    for key_chord in previous {
                                        let command =
                                            Command::UInput(
                                                UInputWorkerCommand::ForwardKeyChord(
                                                    key_chord,
                                                ),
                                            );

                                        match worker_handle
                                            .send_command(command)
                                        {
                                            Ok(_) => {}
                                            Err(_) => break,
                                        }
                                    }
                                }
                                StateMachineResult::Transition() => {}
                            }
                        }
                        _ => {}
                    }

                    match stop_receiver.try_recv() {
                        Ok(()) => {
                            break;
                        }
                        Err(mpsc::TryRecvError::Disconnected) => {
                            break;
                        }
                        Err(mpsc::TryRecvError::Empty) => {}
                    }

                    thread::sleep(Duration::from_millis(10));
                }

                match listener_handle.stop() {
                    Ok(()) => {}
                    Err(()) => {}
                };
            });
        }

        let nia_action_listener_handle =
            NiaActionListenerHandle::new(action_receiver, stop_sender);

        Ok(nia_action_listener_handle)
    }
}
