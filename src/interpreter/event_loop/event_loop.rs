use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread;

use nia_events::Command;

use crate::Interpreter;
use crate::InterpreterCommand;
use crate::CommandResult;
use crate::NiaEventListener;
use crate::Action;

use crate::library;

pub struct EventLoop {}

impl EventLoop {
    pub fn run_event_loop(
        interpreter: Interpreter
    ) -> (mpsc::Sender<InterpreterCommand>, mpsc::Receiver<CommandResult>) {
        let mut interpreter = interpreter;

        let (
            command_sender,
            command_receiver
        ) = mpsc::channel::<InterpreterCommand>();

        let (
            result_sender,
            result_receiver
        ) = mpsc::channel::<CommandResult>();

        let mut event_listener_v: Option<(
            mpsc::Sender<Command>,
            mpsc::Receiver<Action>,
            mpsc::Sender<()>
        )> = None;

        thread::spawn(move || {
            loop {
                match command_receiver.try_recv() {
                    Ok(command) => {
                        match command {
                            InterpreterCommand::Execution(code) => {
                                let result = interpreter.execute(&code);

                                let string_result = match result {
                                    Ok(value) => {
                                        match library::value_to_string(&mut interpreter, value) {
                                            Ok(string) => string,
                                            Err(err) => err.to_string()
                                        }
                                    },
                                    Err(error) => {
                                        error.to_string()
                                    }
                                };

                                match result_sender.send(CommandResult::ExecutionResult(string_result)) {
                                    Ok(()) => {},
                                    Err(_) => break
                                }
                            }
                        }
                    },
                    Err(TryRecvError::Disconnected) => {
                        break;
                    },
                    Err(TryRecvError::Empty) => {}
                };

                if interpreter.is_listening() && event_listener_v.is_none() {
                    let mut event_listener = NiaEventListener::from_interpreter(
                        &mut interpreter
                    );

                    match event_listener.start_listening() {
                        Ok(ok) => {
                            event_listener_v = Some(ok)
                        },
                        Err(error) => break
                    }
                } else if !interpreter.is_listening() && event_listener_v.is_some() {
                    println!("Stopped, probably...");
                    let (_, _, stopper) = event_listener_v.unwrap();

                    stopper.send(());

                    event_listener_v = None;
                }

                if event_listener_v.is_some() {
                    let action_receiver = match &event_listener_v {
                        Some((_, action_receiver, _)) => action_receiver,
                        _ => unreachable!()
                    };

                    loop {
                        match action_receiver.try_recv() {
                            Ok(action) => {
                                match action {
                                    Action::Empty => {},
                                    Action::Execute(value) => {
                                        let root_environment_id = interpreter.get_root_environment();

                                        match interpreter.execute_value(
                                            root_environment_id,
                                            value
                                        ) {
                                            Ok(_) => {},
                                            Err(error) => {
                                                println!("Error happened:");
                                                println!("{}", error);
                                            }
                                        };

                                        println!("State: {}", interpreter.is_listening());
                                    }
                                }
                            },
                            Err(TryRecvError::Empty) => break,
                            Err(TryRecvError::Disconnected) => {
                                // event listener is died somehow, so it won't work anyway
                                // event_listener_v = None;
                                //
                                // interpreter.stop_listening();
                            },
                        }
                    }
                }
            }

            match event_listener_v {
                Some((_, _, stopper)) => {
                    stopper.send(());
                },
                _ => {}
            }
        });

        (command_sender, result_receiver)
    }
}
