use std::sync::mpsc;
use std::thread;

use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use nia_events::UInputWorkerCommand;
use nia_events::WorkerHandle;
use nia_events::XorgWorkerCommand;
use nia_events::{ButtonId, Command, KeyId};

use crate::{Action, ActionDeque, GarbageCollectorWrapper};

use crate::EventLoopHandle;
use crate::NiaActionListener;
use crate::NiaActionListenerHandle;
use crate::NiaChangeMappingCommand;
use crate::NiaChangeMappingCommandResult;
use crate::NiaDefineActionCommand;
use crate::NiaDefineActionCommandResult;
use crate::NiaDefineDeviceCommand;
use crate::NiaDefineDeviceCommandResult;
use crate::NiaDefineMappingCommand;
use crate::NiaDefineMappingCommandResult;
use crate::NiaDefineModifierCommand;
use crate::NiaDefineModifierCommandResult;
use crate::NiaExecuteCodeCommand;
use crate::NiaExecuteCodeCommandResult;
use crate::NiaGetDefinedActionsCommand;
use crate::NiaGetDefinedActionsCommandResult;
use crate::NiaGetDefinedMappingsCommand;
use crate::NiaGetDefinedMappingsCommandResult;
use crate::NiaGetDefinedModifiersCommand;
use crate::NiaGetDefinedModifiersCommandResult;
use crate::NiaInterpreterCommand;
use crate::NiaInterpreterCommandResult;
use crate::NiaIsListeningCommandResult;
use crate::NiaRemoveActionCommand;
use crate::NiaRemoveActionCommandResult;
use crate::NiaRemoveDeviceByNameCommand;
use crate::NiaRemoveDeviceByNameCommandResult;
use crate::NiaRemoveDeviceByPathCommand;
use crate::NiaRemoveDeviceByPathCommandResult;
use crate::NiaRemoveMappingCommand;
use crate::NiaRemoveMappingCommandResult;
use crate::NiaRemoveModifierCommand;
use crate::NiaRemoveModifierCommandResult;
use crate::NiaWorker;
use crate::StateMachineAction;

use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::interpreter::garbage_collector::collect_garbage;
use crate::interpreter::PRIMITIVE_ACTIONS_VARIABLE_NAME;

use crate::library;

pub struct EventLoop {}

const GARBAGE_COLLECTOR_PERIOD: u64 = 120000;

mod do_command {
    pub use super::*;
    use crate::{
        NiaIsListeningCommand, NiaStartListeningCommand,
        NiaStartListeningCommandResult, NiaStopListeningCommand,
        NiaStopListeningCommandResult,
    };

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
        let result =
            library::define_modifier(interpreter, command.get_modifier());
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
        _command: NiaGetDefinedModifiersCommand,
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
        let result = library::remove_modifier(interpreter, command.get_key());
        let result = result.map(|_| String::from("Success"));

        NiaRemoveModifierCommandResult::from(result).into()
    }

    fn do_command_get_defined_actions(
        interpreter: &mut Interpreter,
        _command: NiaGetDefinedActionsCommand,
    ) -> NiaInterpreterCommandResult {
        let result = library::get_defined_actions(interpreter);

        NiaGetDefinedActionsCommandResult::from(result).into()
    }

    fn do_command_define_action(
        interpreter: &mut Interpreter,
        command: NiaDefineActionCommand,
    ) -> NiaInterpreterCommandResult {
        let mut command = command;
        let action = command.take_action();
        let action_name = action.get_action_name();

        let result = match action.get_action() {
            Action::KeyPress(key_code) => library::define_action_key_press(
                interpreter,
                action_name,
                *key_code,
            ),
            Action::KeyClick(key_code) => library::define_action_key_click(
                interpreter,
                action_name,
                *key_code,
            ),
            Action::KeyRelease(key_code) => library::define_action_key_release(
                interpreter,
                action_name,
                *key_code,
            ),
            Action::MouseAbsoluteMove(x, y) => {
                library::define_action_mouse_absolute_move(
                    interpreter,
                    action_name,
                    *x,
                    *y,
                )
            }
            Action::MouseRelativeMove(dx, dy) => {
                library::define_action_mouse_relative_move(
                    interpreter,
                    action_name,
                    *dx,
                    *dy,
                )
            }
            Action::TextType(text_to_type) => library::define_action_text_type(
                interpreter,
                action_name,
                text_to_type,
            ),
            Action::Wait(ms_amount) => library::define_action_wait(
                interpreter,
                action_name,
                *ms_amount,
            ),
            Action::ExecuteCode(code_to_execute) => {
                library::define_action_execute_code(
                    interpreter,
                    action_name,
                    code_to_execute,
                )
            }
            Action::ExecuteFunction(function_name) => {
                library::define_action_execute_function(
                    interpreter,
                    action_name,
                    function_name,
                )
            }
            Action::ExecuteOSCommand(os_command) => {
                library::define_action_execute_os_command(
                    interpreter,
                    action_name,
                    os_command,
                )
            }
            Action::ExecuteFunctionValue(_) => panic!(
                "Invariant violation Action::ExecuteFunctionValue must not be able to be defined."
            ),
        };

        let result = result.map(|_| String::from("Success"));

        NiaDefineActionCommandResult::from(result).into()
    }

    fn do_command_remove_action(
        interpreter: &mut Interpreter,
        command: NiaRemoveActionCommand,
    ) -> NiaInterpreterCommandResult {
        let result =
            library::remove_action(interpreter, command.get_action_name());
        let result = result.map(|_| String::from("Success"));

        NiaRemoveActionCommandResult::from(result).into()
    }

    fn do_command_get_defined_mappings(
        interpreter: &mut Interpreter,
        command: NiaGetDefinedMappingsCommand,
    ) -> NiaInterpreterCommandResult {
        let result = library::get_defined_mappings(interpreter);

        NiaGetDefinedMappingsCommandResult::from(result).into()
    }

    fn do_command_define_mapping(
        interpreter: &mut Interpreter,
        command: NiaDefineMappingCommand,
    ) -> NiaInterpreterCommandResult {
        let result =
            library::define_global_mapping(interpreter, command.get_mapping());
        let result = result.map(|_| String::from("Success"));

        NiaDefineMappingCommandResult::from(result).into()
    }

    fn do_command_change_mapping(
        interpreter: &mut Interpreter,
        command: NiaChangeMappingCommand,
    ) -> NiaInterpreterCommandResult {
        let result = library::change_global_mapping(
            interpreter,
            command.get_key_chords(),
            command.get_action(),
        );
        let result = result.map(|_| String::from("Success"));

        NiaChangeMappingCommandResult::from(result).into()
    }

    fn do_command_remove_mapping(
        interpreter: &mut Interpreter,
        command: NiaRemoveMappingCommand,
    ) -> NiaInterpreterCommandResult {
        let result = library::remove_global_mapping(
            interpreter,
            command.get_key_chords(),
        );
        let result = result.map(|_| String::from("Success"));

        NiaRemoveMappingCommandResult::from(result).into()
    }

    fn do_command_is_listening(
        interpreter: &mut Interpreter,
        command: NiaIsListeningCommand,
    ) -> NiaInterpreterCommandResult {
        let result = Ok(interpreter.is_listening());

        NiaIsListeningCommandResult::from(result).into()
    }

    fn do_command_start_listening(
        interpreter: &mut Interpreter,
        command: NiaStartListeningCommand,
    ) -> NiaInterpreterCommandResult {
        let result = interpreter.start_listening();
        let result = result.map(|_| String::from("Success"));

        NiaStartListeningCommandResult::from(result).into()
    }

    fn do_command_stop_listening(
        interpreter: &mut Interpreter,
        command: NiaStopListeningCommand,
    ) -> NiaInterpreterCommandResult {
        let result = interpreter.stop_listening();
        let result = result.map(|_| String::from("Success"));

        NiaStopListeningCommandResult::from(result).into()
    }

    pub fn do_command(
        interpreter: &mut Interpreter,
        command: NiaInterpreterCommand,
    ) -> NiaInterpreterCommandResult {
        match command {
            NiaInterpreterCommand::DefineDevice(command) => {
                do_command_define_keyboard(interpreter, command)
            }
            NiaInterpreterCommand::DefineModifier(command) => {
                do_command_define_modifier(interpreter, command)
            }
            NiaInterpreterCommand::ExecuteCode(command) => {
                do_command_execute_code(interpreter, command)
            }
            NiaInterpreterCommand::GetDefinedModifiers(command) => {
                do_command_get_defined_modifiers(interpreter, command)
            }
            NiaInterpreterCommand::RemoveDeviceByPath(command) => {
                do_command_remove_keyboard_by_path(interpreter, command)
            }
            NiaInterpreterCommand::RemoveDefineDeviceByName(command) => {
                do_command_remove_keyboard_by_name(interpreter, command)
            }
            NiaInterpreterCommand::RemoveModifier(command) => {
                do_command_remove_modifier(interpreter, command)
            }
            NiaInterpreterCommand::GetDefinedActions(command) => {
                do_command_get_defined_actions(interpreter, command)
            }
            NiaInterpreterCommand::DefineAction(command) => {
                do_command_define_action(interpreter, command)
            }
            NiaInterpreterCommand::RemoveAction(command) => {
                do_command_remove_action(interpreter, command)
            }
            NiaInterpreterCommand::GetDefinedMappings(command) => {
                do_command_get_defined_mappings(interpreter, command)
            }
            NiaInterpreterCommand::DefineMapping(command) => {
                do_command_define_mapping(interpreter, command)
            }
            NiaInterpreterCommand::ChangeMapping(command) => {
                do_command_change_mapping(interpreter, command)
            }
            NiaInterpreterCommand::RemoveMapping(command) => {
                do_command_remove_mapping(interpreter, command)
            }
            NiaInterpreterCommand::IsListening(command) => {
                do_command_is_listening(interpreter, command)
            }
            NiaInterpreterCommand::StartListening(command) => {
                do_command_start_listening(interpreter, command)
            }
            NiaInterpreterCommand::StopListening(command) => {
                do_command_stop_listening(interpreter, command)
            }
        }
    }
}

impl EventLoop {
    fn take_actions_from_interpreter(
        interpreter: &mut Interpreter,
    ) -> Result<Vec<Action>, Error> {
        let action_list_value = library::get_root_variable(
            interpreter,
            PRIMITIVE_ACTIONS_VARIABLE_NAME,
        )?;

        let action_vector =
            library::read_as_vector(interpreter, action_list_value)?;

        let actions = action_vector
            .into_iter()
            .rev()
            .map(|action_value| {
                library::list_to_action(interpreter, action_value)
            })
            .collect::<Result<Vec<Action>, Error>>();

        let nil = interpreter.intern_nil_symbol_value();
        library::set_root_variable(
            interpreter,
            PRIMITIVE_ACTIONS_VARIABLE_NAME,
            nil,
        )?;

        actions
    }

    fn handle_action(
        interpreter: &mut Interpreter,
        action: Action,
    ) -> Result<Option<Command>, Error> {
        let command = match action {
            Action::KeyPress(key_code) => Command::UInput(
                UInputWorkerCommand::KeyDown(KeyId::new(key_code as u16)),
            ),
            Action::KeyClick(key_code) => Command::UInput(
                UInputWorkerCommand::KeyPress(KeyId::new(key_code as u16)),
            ),
            Action::KeyRelease(key_code) => Command::UInput(
                UInputWorkerCommand::KeyUp(KeyId::new(key_code as u16)),
            ),
            Action::MouseButtonPress(button_code) => {
                Command::UInput(UInputWorkerCommand::MouseButtonDown(
                    ButtonId::new(button_code as u16),
                ))
            }
            Action::MouseButtonClick(button_code) => {
                Command::UInput(UInputWorkerCommand::MouseButtonPress(
                    ButtonId::new(button_code as u16),
                ))
            }
            Action::MouseButtonRelease(button_code) => {
                Command::UInput(UInputWorkerCommand::MouseButtonUp(
                    ButtonId::new(button_code as u16),
                ))
            }
            Action::MouseAbsoluteMove(x, y) => Command::Xorg(
                XorgWorkerCommand::MouseMoveTo(x as i16, y as i16),
            ),
            Action::MouseRelativeMove(dx, dy) => Command::Xorg(
                XorgWorkerCommand::MouseMoveBy(dx as i16, dy as i16),
            ),
            Action::TextType(text) => {
                Command::Xorg(XorgWorkerCommand::TextType(text))
            }
            Action::ExecuteOSCommand(os_command) => Command::Spawn(os_command),
            Action::ExecuteCode(code) => {
                interpreter.execute_in_main_environment(&code)?;

                return Ok(None);
            }
            Action::ExecuteFunction(function_name) => {
                let main_environment_id = interpreter.get_main_environment_id();
                let symbol_id = interpreter.intern_symbol_id(&function_name);

                let function = interpreter
                    .lookup_function(main_environment_id, symbol_id)?;

                match function {
                    Some(value) => {
                        interpreter.execute_function_without_arguments_int_main_environment(value)?;
                    }
                    None => {
                        return Error::generic_execution_error(
                            "Cannot find function",
                        )
                        .into();
                    }
                }

                return Ok(None);
            }
            Action::ExecuteFunctionValue(function_value) => {
                interpreter
                    .execute_function_without_arguments_int_main_environment(
                        function_value,
                    )?;

                return Ok(None);
            }
            Action::Wait(_) => return Ok(None),
        };

        Ok(Some(command))
    }

    pub fn run_event_loop(interpreter: Interpreter) -> EventLoopHandle {
        let mut interpreter = interpreter;
        let mut gc = GarbageCollectorWrapper::new(GARBAGE_COLLECTOR_PERIOD);

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

            let mut action_deque = ActionDeque::new();

            loop {
                // execute command that was received with channel
                match interpreter_command_receiver.try_recv() {
                    Ok(command) => {
                        let command_result =
                            do_command::do_command(&mut interpreter, command);

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
                    let action_listener =
                        match NiaActionListener::from_interpreter(
                            &mut interpreter,
                        ) {
                            Ok(action_listener) => action_listener,
                            Err(_error) => {
                                break;
                            }
                        };

                    match action_listener.start_listening(worker_handle.clone())
                    {
                        Ok(ok) => {
                            action_listener_handle = Some(ok);
                        }
                        Err(_error) => {
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

                // add actions to queue from state machine
                if let Some(handle) = &action_listener_handle {
                    loop {
                        match handle.try_receive_action() {
                            Ok(action) => match action {
                                StateMachineAction::Empty => {}
                                StateMachineAction::Execute(action) => {
                                    action_deque.push_action(action);
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

                // add actions to queue from interpreter
                let actions =
                    EventLoop::take_actions_from_interpreter(&mut interpreter)
                        .expect("");

                action_deque.push_actions(actions);

                // handle actions from queue
                while let Some(action) = action_deque.take_action() {
                    match EventLoop::handle_action(&mut interpreter, action) {
                        Ok(Some(command)) => {
                            match worker_handle.send_command(command) {
                                Ok(_) => {}
                                Err(error) => {
                                    // worker is dead
                                }
                            }
                        }
                        Ok(None) => {}
                        Err(error) => {
                            println!("{:?}", error);

                            if error.is_failure() {
                                // handle failure
                            }
                        }
                    };
                }

                // collect garbage
                let was_collected = match gc.probably_collect(&mut interpreter)
                {
                    Ok(was_collected) => was_collected,
                    Err(error) => {
                        println!("{:?}", error);

                        if error.is_failure() {
                            // handle failure
                        }
                        false
                    }
                };

                if !was_collected {
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
