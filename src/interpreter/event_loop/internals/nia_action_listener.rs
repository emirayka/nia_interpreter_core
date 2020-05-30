use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use nia_events::Command;
use nia_events::Event;
use nia_events::Listener;
use nia_events::ListenerSettings;
use nia_events::ListenerSettingsBuilder;
use nia_events::UInputWorkerCommand;
use nia_events::WorkerHandle;

use nia_state_machine::StateMachine;
use nia_state_machine::StateMachineResult;

use crate::Interpreter;
use crate::Key;
use crate::KeyChord;
use crate::NiaActionListenerHandle;
use crate::StateMachineAction;
use crate::{Convertable, Error};
use crate::{DeviceInfo, Mapping, ModifierDescription};

use crate::library;

pub struct NiaActionListener {
    devices: Vec<DeviceInfo>,
    modifiers: Vec<ModifierDescription>,
    mappings: Vec<Mapping>,
}

impl NiaActionListener {
    pub fn new() -> NiaActionListener {
        NiaActionListener {
            devices: Vec::new(),
            modifiers: Vec::new(),
            mappings: Vec::new(),
        }
    }

    pub fn from_interpreter(
        interpreter: &mut Interpreter,
    ) -> Result<NiaActionListener, Error> {
        let devices_info = library::get_defined_devices_info(interpreter)?;
        let modifiers = library::get_defined_modifiers(interpreter)?;
        let mappings = library::get_defined_mappings(interpreter)?;

        let nia_action_listener = NiaActionListener {
            devices: devices_info,
            modifiers,
            mappings,
        };

        Ok(nia_action_listener)
    }

    fn build_settings(&self) -> ListenerSettings {
        let mut settings_builder = ListenerSettingsBuilder::new();

        for device_info in &self.devices {
            settings_builder = settings_builder.add_device(
                device_info.get_path().clone(),
                device_info.get_id() as u16,
            );
        }

        for modifier in &self.modifiers {
            match modifier.get_key() {
                Key::LoneKey(lone_key) => {
                    settings_builder = settings_builder
                        .add_modifier_1(lone_key.get_key_id() as u16);
                }
                Key::DeviceKey(device_key) => {
                    settings_builder = settings_builder.add_modifier_2(
                        device_key.get_device_id() as u16,
                        device_key.get_key_id() as u16,
                    );
                }
            }
        }

        settings_builder.build()
    }

    fn construct_state_machine(
        &self,
    ) -> Result<StateMachine<KeyChord, StateMachineAction>, Error> {
        let mut state_machine = nia_state_machine::StateMachine::new();

        for mapping in &self.mappings {
            let key_chords = mapping.get_key_chords().clone();
            let state_machine_action =
                StateMachineAction::Execute(mapping.get_action().clone());

            state_machine
                .add(key_chords, state_machine_action)
                .map_err(|_| {
                    Error::generic_execution_error("Cannot add mapping")
                })?;
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

        let (state_machine_action_sender, state_machine_action_receiver) =
            mpsc::channel();
        let (stop_sender, stop_receiver) = mpsc::channel();

        {
            let action_sender = state_machine_action_sender;
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
                        Some(Event::KeyChordEvent(key_chord_et)) => {
                            let key_chord =
                                KeyChord::from_nia_events_representation(
                                    &key_chord_et,
                                );

                            match state_machine.excite(key_chord) {
                                StateMachineResult::Excited(action) => {
                                    match action_sender.send(action) {
                                        Ok(_) => {}
                                        Err(_) => {}
                                    }
                                }
                                StateMachineResult::Fallback(previous) => {
                                    for key_chord in previous {
                                        let command =
                                            Command::UInput(
                                                UInputWorkerCommand::ForwardKeyChord(
                                                    key_chord.to_nia_events_representation(),
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

        let nia_action_listener_handle = NiaActionListenerHandle::new(
            state_machine_action_receiver,
            stop_sender,
        );

        Ok(nia_action_listener_handle)
    }
}
