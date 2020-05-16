use std::sync::mpsc;
use std::thread;

use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use nia_events::ButtonId;
use nia_events::Command;
use nia_events::KeyId;
use nia_events::UInputWorkerCommand;
use nia_events::WorkerHandle;
use nia_events::XorgWorkerCommand;

use crate::NiaInterpreterCommand;
use crate::NiaInterpreterCommandResult;
use crate::NiaWorker;
use crate::Value;
use crate::{Action, NiaActionListenerHandle};
use crate::{
    Error, NiaDefineDeviceCommand, NiaExecuteCodeCommand,
    NiaExecuteCodeCommandResult,
};
use crate::{EventLoopHandle, NiaDefineModifierCommand};
use crate::{
    Interpreter, NiaDefineDeviceCommandResult, NiaDefineModifierCommandResult,
    NiaGetDefinedModifiersCommand, NiaRemoveDeviceByNameCommand,
    NiaRemoveDeviceByNameCommandResult, NiaRemoveDeviceByPathCommand,
    NiaRemoveDeviceByPathCommandResult, NiaRemoveModifierCommand,
    NiaRemoveModifierCommandResult,
};
use crate::{NiaActionListener, NiaGetDefinedModifiersCommandResult};

use crate::interpreter::garbage_collector::collect_garbage;
use crate::library;

pub struct EventLoop {}

const GARBAGE_COLLECTOR_PERIOD: u64 = 120000;

mod send_events {
    use super::*;

    fn make_key_down_command(
        event_vector: Vec<Value>,
    ) -> Result<Command, Error> {
        let key_id = library::read_key_id_from_vector(event_vector)?;

        Ok(Command::UInput(UInputWorkerCommand::KeyDown(key_id)))
    }

    fn make_key_press_command(
        event_vector: Vec<Value>,
    ) -> Result<Command, Error> {
        let key_id = library::read_key_id_from_vector(event_vector)?;

        Ok(Command::UInput(UInputWorkerCommand::KeyPress(key_id)))
    }

    fn make_key_up_command(event_vector: Vec<Value>) -> Result<Command, Error> {
        let key_id = library::read_key_id_from_vector(event_vector)?;

        Ok(Command::UInput(UInputWorkerCommand::KeyUp(key_id)))
    }

    fn make_mouse_button_down_command(
        event_vector: Vec<Value>,
    ) -> Result<Command, Error> {
        let button_id = library::read_button_id_from_vector(event_vector)?;

        Ok(Command::UInput(UInputWorkerCommand::MouseButtonDown(
            button_id,
        )))
    }

    fn make_mouse_button_press_command(
        event_vector: Vec<Value>,
    ) -> Result<Command, Error> {
        let button_id = library::read_button_id_from_vector(event_vector)?;

        Ok(Command::UInput(UInputWorkerCommand::MouseButtonPress(
            button_id,
        )))
    }

    fn make_mouse_button_up_command(
        event_vector: Vec<Value>,
    ) -> Result<Command, Error> {
        let button_id = library::read_button_id_from_vector(event_vector)?;

        Ok(Command::UInput(UInputWorkerCommand::MouseButtonUp(
            button_id,
        )))
    }

    fn make_mouse_button_move_by_command(
        event_vector: Vec<Value>,
    ) -> Result<Command, Error> {
        if event_vector.len() != 2 {
            return Error::invalid_argument_error(
                "Cannot parse event, not enough items",
            )
            .into();
        }

        let mut event_vector = event_vector;

        let x = library::read_as_i64(event_vector.remove(0))?;
        let y = library::read_as_i64(event_vector.remove(0))?;

        Ok(Command::Xorg(XorgWorkerCommand::MouseMoveBy(
            x as i16, y as i16,
        )))
    }

    fn make_mouse_button_move_to_command(
        event_vector: Vec<Value>,
    ) -> Result<Command, Error> {
        if event_vector.len() != 2 {
            return Error::invalid_argument_error(
                "Cannot parse event, not enough items",
            )
            .into();
        }

        let mut event_vector = event_vector;

        let x = library::read_as_i64(event_vector.remove(0))?;
        let y = library::read_as_i64(event_vector.remove(0))?;

        if x < 0 || y < 0 {
            return Error::invalid_argument_error(
                "Invalid coordinate specification.",
            )
            .into();
        }

        Ok(Command::Xorg(XorgWorkerCommand::MouseMoveTo(
            x as i16, y as i16,
        )))
    }

    fn make_type_text_command(
        interpreter: &mut Interpreter,
        event_vector: Vec<Value>,
    ) -> Result<Command, Error> {
        let mut event_vector = event_vector;

        let text =
            library::read_as_string(interpreter, event_vector.remove(0))?
                .clone();

        Ok(Command::Xorg(XorgWorkerCommand::TextType(text)))
    }

    fn make_wait_command(event_vector: Vec<Value>) -> Result<Command, Error> {
        let mut event_vector = event_vector;

        let milliseconds = match event_vector.remove(0) {
            Value::Integer(ms) => {
                if ms > 0 {
                    ms as u64
                } else {
                    return Error::invalid_argument_error(
                        "Expected duration to be not positive.",
                    )
                    .into();
                }
            }
            v => {
                return Error::invalid_argument_error(
                    "Unknown value passed as wait.",
                )
                .into();
            }
        };

        Ok(Command::Wait(milliseconds))
    }

    fn send_event(
        interpreter: &mut Interpreter,
        event_vector: Value,
        worker_handle: &WorkerHandle,
    ) -> Result<(), Error> {
        let mut event_vector =
            library::read_as_vector(interpreter, event_vector)?;

        if event_vector.len() == 0 {
            return Error::generic_execution_error("Event vector is empty.")
                .into();
        }

        let event_name_symbol_id =
            library::read_as_symbol_id(event_vector.remove(0))?;

        let event_name =
            interpreter.get_symbol_name(event_name_symbol_id)?.clone();

        let command = match event_name.as_str() {
            "key-down" => make_key_down_command(event_vector)?,
            "key-press" => make_key_press_command(event_vector)?,
            "key-up" => make_key_up_command(event_vector)?,

            "mouse-button-down" => {
                make_mouse_button_down_command(event_vector)?
            }
            "mouse-button-press" => {
                make_mouse_button_press_command(event_vector)?
            }
            "mouse-button-up" => make_mouse_button_up_command(event_vector)?,

            "mouse-move-by" => make_mouse_button_move_by_command(event_vector)?,
            "mouse-move-to" => make_mouse_button_move_to_command(event_vector)?,

            "text-type" => make_type_text_command(interpreter, event_vector)?,
            "wait" => make_wait_command(event_vector)?,

            _ => return Error::invalid_argument_error("Unknown action").into(),
        };

        println!("{:?}", command);

        match worker_handle.send_command(command) {
            Ok(_) => Ok(()),
            Err(_) => Ok(()),
        }
    }

    pub fn send_events(
        interpreter: &mut Interpreter,
        worker_handle: &WorkerHandle,
    ) -> Result<(), Error> {
        let actions_value =
            library::get_root_variable(interpreter, "--actions")?;

        let events_vectors =
            library::read_as_vector(interpreter, actions_value)?
                .into_iter()
                .rev()
                .collect::<Vec<Value>>();

        let nil = interpreter.intern_nil_symbol_value();
        library::set_root_variable(interpreter, "--actions", nil)?;

        for event_vector in events_vectors {
            send_event(interpreter, event_vector, worker_handle)?;
        }

        Ok(())
    }
}

impl EventLoop {
    fn do_command_define_keyboard(
        interpreter: &mut Interpreter,
        command: NiaDefineDeviceCommand,
    ) -> NiaInterpreterCommandResult {
        let result = library::define_device(
            interpreter,
            command.get_device_id(),
            command.get_device_path(),
            command.get_device_name(),
        );

        let result = result.map(|_| String::from("Success"));

        NiaDefineDeviceCommandResult::from(result).into()
    }

    fn do_command_define_modifier(
        interpreter: &mut Interpreter,
        command: NiaDefineModifierCommand,
    ) -> NiaInterpreterCommandResult {
        let result = library::define_modifier(
            interpreter,
            command.get_device_id(),
            command.get_key_code(),
            command.get_modifier_alias(),
        );
        let result = result.map(|_| String::from("Success"));

        NiaDefineModifierCommandResult::from(result).into()
    }

    fn do_command_execute_code(
        interpreter: &mut Interpreter,
        command: NiaExecuteCodeCommand,
    ) -> NiaInterpreterCommandResult {
        let result =
            interpreter.execute_in_main_environment(command.get_code());

        let result = match result {
            Ok(value) => library::value_to_string(interpreter, value),
            Err(error) => Err(error),
        };

        NiaExecuteCodeCommandResult::from(result).into()
    }

    fn do_command_get_defined_modifiers(
        interpreter: &mut Interpreter,
        command: NiaGetDefinedModifiersCommand,
    ) -> NiaInterpreterCommandResult {
        let result = library::get_defined_modifiers(interpreter);

        NiaGetDefinedModifiersCommandResult::from(result).into()
    }

    fn do_command_remove_keyboard_by_path(
        interpreter: &mut Interpreter,
        command: NiaRemoveDeviceByPathCommand,
    ) -> NiaInterpreterCommandResult {
        let result = library::remove_keyboard_by_path_with_string(
            interpreter,
            command.get_device_path(),
        );
        let result = result.map(|_| String::from("Success"));

        NiaRemoveDeviceByPathCommandResult::from(result).into()
    }

    fn do_command_remove_keyboard_by_name(
        interpreter: &mut Interpreter,
        command: NiaRemoveDeviceByNameCommand,
    ) -> NiaInterpreterCommandResult {
        let result = library::remove_keyboard_by_name_with_string(
            interpreter,
            command.get_device_name(),
        );
        let result = result.map(|_| String::from("Success"));

        NiaRemoveDeviceByNameCommandResult::from(result).into()
    }

    fn do_command_remove_modifier(
        interpreter: &mut Interpreter,
        command: NiaRemoveModifierCommand,
    ) -> NiaInterpreterCommandResult {
        let result = library::remove_modifier(
            interpreter,
            command.get_device_id(),
            command.get_key_code(),
        );
        let result = result.map(|_| String::from("Success"));

        NiaRemoveModifierCommandResult::from(result).into()
    }

    pub fn run_event_loop(interpreter: Interpreter) -> EventLoopHandle {
        let mut interpreter = interpreter;

        let (interpreter_command_sender, interpreter_command_receiver) =
            mpsc::channel::<NiaInterpreterCommand>();

        let (
            interpreter_command_result_sender,
            interpreter_command_result_receiver,
        ) = mpsc::channel::<NiaInterpreterCommandResult>();

        thread::spawn(move || {
            let worker_handle = NiaWorker::new().start_sending().expect("");
            // todo: change
            let mut action_listener_handle: Option<NiaActionListenerHandle> =
                None;
            let current_time = SystemTime::now();

            let mut time_for_garbage_collection = current_time
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

            time_for_garbage_collection +=
                Duration::from_millis(GARBAGE_COLLECTOR_PERIOD);

            loop {
                // execute command that was received with channel
                match interpreter_command_receiver.try_recv() {
                    Ok(command) => {
                        let command_result = match command {
                            NiaInterpreterCommand::DefineDevice(command) => {
                                EventLoop::do_command_define_keyboard(
                                    &mut interpreter,
                                    command,
                                )
                            }
                            NiaInterpreterCommand::DefineModifier(command) => {
                                EventLoop::do_command_define_modifier(
                                    &mut interpreter,
                                    command,
                                )
                            }
                            NiaInterpreterCommand::ExecuteCode(command) => {
                                EventLoop::do_command_execute_code(
                                    &mut interpreter,
                                    command,
                                )
                            }
                            NiaInterpreterCommand::GetDefinedModifiers(
                                command,
                            ) => EventLoop::do_command_get_defined_modifiers(
                                &mut interpreter,
                                command,
                            ),
                            NiaInterpreterCommand::RemoveDeviceByPath(
                                command,
                            ) => EventLoop::do_command_remove_keyboard_by_path(
                                &mut interpreter,
                                command,
                            ),
                            NiaInterpreterCommand::RemoveDefineDeviceByName(
                                command,
                            ) => EventLoop::do_command_remove_keyboard_by_name(
                                &mut interpreter,
                                command,
                            ),
                            NiaInterpreterCommand::RemoveModifier(command) => {
                                EventLoop::do_command_remove_modifier(
                                    &mut interpreter,
                                    command,
                                )
                            }
                        };

                        match interpreter_command_result_sender
                            .send(command_result)
                        {
                            Ok(()) => {}
                            Err(_) => break,
                        }
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                };

                // construct/stop key remapping threads
                if interpreter.is_listening()
                    && action_listener_handle.is_none()
                {
                    let mut action_listener =
                        match NiaActionListener::from_interpreter(
                            &mut interpreter,
                        ) {
                            Ok(action_listener) => action_listener,
                            Err(error) => {
                                break;
                            }
                        };

                    match action_listener.start_listening(worker_handle.clone())
                    {
                        Ok(ok) => {
                            action_listener_handle = Some(ok);
                        }
                        Err(error) => {
                            break;
                        }
                    }
                } else if !interpreter.is_listening()
                    && action_listener_handle.is_some()
                {
                    match &action_listener_handle {
                        Some(handle) => {
                            handle.stop();
                        }
                        None => {}
                    }

                    action_listener_handle = None;
                }

                // send actions to worker
                match &action_listener_handle {
                    Some(handle) => {
                        loop {
                            match handle.try_receive_action() {
                                Ok(action) => match action {
                                    Action::Empty => {}
                                    Action::Execute(value) => {
                                        match interpreter
                                            .execute_function_without_arguments(
                                                value,
                                            ) {
                                            Ok(_) => {}
                                            Err(error) => {
                                                println!("Error happened:");
                                                println!("{}", error);
                                            }
                                        };
                                    }
                                },
                                Err(mpsc::TryRecvError::Empty) => {
                                    break;
                                }
                                Err(mpsc::TryRecvError::Disconnected) => {
                                    // event listener is died somehow, so it won't work anyway
                                    // event_listener_v = None;
                                    //
                                    // interpreter.stop_listening();
                                }
                            }
                        }
                    }
                    _ => {}
                }

                // send events for execution
                send_events::send_events(&mut interpreter, &worker_handle)
                    .expect("Sending events failed.");

                // collect garbage
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards.");

                if current_time >= time_for_garbage_collection {
                    time_for_garbage_collection = current_time
                        + Duration::from_millis(GARBAGE_COLLECTOR_PERIOD);

                    collect_garbage(&mut interpreter)
                        .expect("Garbage collection failed.");
                } else {
                    thread::sleep(Duration::from_millis(10));
                }
            }

            match action_listener_handle {
                Some(handle) => {
                    handle.stop();
                }
                _ => {}
            }

            match worker_handle.stop() {
                Ok(()) => {}
                Err(()) => {}
            }
        });

        let event_loop_handle = EventLoopHandle::new(
            interpreter_command_sender,
            interpreter_command_result_receiver,
        );

        event_loop_handle
    }
}
