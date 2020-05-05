use std::sync::mpsc;
use std::sync::mpsc::{Sender, TryRecvError};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use nia_events::UinputCommand;
use nia_events::{ButtonId, Command, KeyId};

use crate::Action;
use crate::CommandResult;
use crate::InterpreterCommand;
use crate::NiaEventListener;
use crate::{ExecutionResult, Interpreter, NiaCommandSender, Value};

use crate::interpreter::garbage_collector::collect_garbage;
use crate::library;
use crate::parser::prefixed_element::Prefix::GraveAccent;
use std::time::Duration;

pub struct EventLoop {}

const GARBAGE_COLLECTOR_PERIOD: u64 = 120000;

fn parse_key_id_from_vector(
    interpreter: &mut Interpreter,
    event_vector: Vec<Value>,
) -> Result<KeyId, ()> {
    if event_vector.len() == 0 {
        println!("Invalid event description, expected key id");
        return Err(());
    }

    let mut event_vector = event_vector;

    let key_id = match library::read_as_i64(event_vector.remove(0)) {
        Ok(key_id) => key_id,
        Err(error) => {
            println!("Cannot parse key id, caused by:");
            println!("{}", error);
            return Err(());
        },
    };

    Ok(KeyId::new(key_id as u16))
}

fn parse_button_id_from_vector(
    interpreter: &mut Interpreter,
    event_vector: Vec<Value>,
) -> Result<ButtonId, ()> {
    if event_vector.len() == 0 {
        println!("Invalid event description, expected button id");
        return Err(());
    }

    let mut event_vector = event_vector;

    let button_id = match library::read_as_i64(event_vector.remove(0)) {
        Ok(button_id) => {
            if button_id > 0 && button_id <= 8 {
                button_id
            } else {
                println!(
                    "Invalid button id, expected interval [1-8], got: {}",
                    button_id
                );
                return Err(());
            }
        },
        Err(error) => {
            println!("Cannot parse button id, caused by:");
            println!("{}", error);
            return Err(());
        },
    };

    Ok(ButtonId::new(button_id as u16))
}

fn send_events(
    interpreter: &mut Interpreter,
    cmd_sender: &Sender<Command>,
) -> Result<(), ()> {
    let actions_value =
        match library::get_root_variable(interpreter, "--actions") {
            Ok(actions_value) => actions_value,
            Err(error) => {
                println!("Cannot parse event queue, caused by: ");
                println!("{}", error);
                return Err(());
            },
        };

    let events_vector =
        match library::read_as_vector(interpreter, actions_value) {
            Ok(vector) => vector.into_iter().rev().collect::<Vec<Value>>(),
            Err(error) => {
                println!("Cannot parse event queue, caused by: ");
                println!("{}", error);
                return Err(());
            },
        };

    let nil = interpreter.intern_nil_symbol_value();
    match library::set_root_variable(interpreter, "--actions", nil) {
        Ok(_) => {},
        Err(error) => {
            println!("Cannot clear event queue, caused by: ");
            println!("{}", error);
            return Err(());
        },
    };

    for event in events_vector {
        let mut event_vector = match library::read_as_vector(interpreter, event)
        {
            Ok(vector) => vector,
            Err(error) => {
                println!("Cannot parse event, caused by: ");
                println!("{}", error);
                continue;
            },
        };

        if event_vector.len() == 0 {
            println!("Invalid event description, expected event name");
            continue;
        }

        let event_name =
            match library::read_as_symbol_id(event_vector.remove(0)) {
                Ok(symbol_id) => match interpreter.get_symbol_name(symbol_id) {
                    Ok(symbol_name) => symbol_name.clone(),
                    Err(error) => {
                        println!("Cannot parse event name");
                        println!("{}", error);
                        continue;
                    },
                },
                Err(error) => {
                    println!("Cannot parse event, caused by: ");
                    println!("{}", error);
                    continue;
                },
            };

        let command = if event_name == "key-down" {
            let key_id =
                match parse_key_id_from_vector(interpreter, event_vector) {
                    Ok(key_id) => key_id,
                    Err(()) => continue,
                };

            Command::UinputCommand(UinputCommand::KeyDown(key_id))
        } else if event_name == "key-press" {
            let key_id =
                match parse_key_id_from_vector(interpreter, event_vector) {
                    Ok(key_id) => key_id,
                    Err(()) => continue,
                };

            Command::UinputCommand(UinputCommand::KeyPress(key_id))
        } else if event_name == "key-up" {
            let key_id =
                match parse_key_id_from_vector(interpreter, event_vector) {
                    Ok(key_id) => key_id,
                    Err(()) => continue,
                };

            Command::UinputCommand(UinputCommand::KeyUp(key_id))
        } else if event_name == "mouse-button-down" {
            let button_id =
                match parse_button_id_from_vector(interpreter, event_vector) {
                    Ok(key_id) => key_id,
                    Err(()) => continue,
                };

            Command::UinputCommand(UinputCommand::MouseButtonDown(button_id))
        } else if event_name == "mouse-button-press" {
            let button_id =
                match parse_button_id_from_vector(interpreter, event_vector) {
                    Ok(key_id) => key_id,
                    Err(()) => continue,
                };

            Command::UinputCommand(UinputCommand::MouseButtonPress(button_id))
        } else if event_name == "mouse-button-up" {
            let button_id =
                match parse_button_id_from_vector(interpreter, event_vector) {
                    Ok(key_id) => key_id,
                    Err(()) => continue,
                };

            Command::UinputCommand(UinputCommand::MouseButtonUp(button_id))
        } else if event_name == "mouse-move-by" {
            if event_vector.len() != 2 {
                println!("Cannot parse event, not enough items");
                continue;
            }
            let x = match library::read_as_i64(event_vector.remove(0)) {
                Ok(x) => x,
                Err(error) => {
                    println!("Cannot parse event, invalid value");
                    println!("{}", error);
                    continue;
                },
            };
            let y = match library::read_as_i64(event_vector.remove(0)) {
                Ok(x) => x,
                Err(error) => {
                    println!("Cannot parse event, invalid value");
                    println!("{}", error);
                    continue;
                },
            };
            Command::UinputCommand(UinputCommand::MouseMoveBy(
                x as i16, y as i16,
            ))
        } else if event_name == "mouse-move-to" {
            if event_vector.len() != 2 {
                println!("Cannot parse event, not enough items");
                continue;
            }

            let x = match library::read_as_i64(event_vector.remove(0)) {
                Ok(x) => x,
                Err(error) => {
                    println!("Cannot parse event, invalid value");
                    println!("{}", error);
                    continue;
                },
            };
            let y = match library::read_as_i64(event_vector.remove(0)) {
                Ok(x) => x,
                Err(error) => {
                    println!("Cannot parse event, invalid value");
                    println!("{}", error);
                    continue;
                },
            };

            if x < 0 || y < 0 {
                println!(
                    "Invalid move to event description. Expected two positive coordinates. Got: {}, {}",
                    x, y
                );
                continue;
            }

            Command::UinputCommand(UinputCommand::MouseMoveTo(
                x as i16, y as i16,
            ))
        } else if event_name == "text-type" {
            if event_vector.len() != 1 {
                println!("Cannot parse event, not enough items");
                continue;
            }

            let text = match library::read_as_string(
                interpreter,
                event_vector.remove(0),
            ) {
                Ok(string) => string.clone(),
                Err(error) => {
                    println!("Cannot parse event cause:");
                    println!("{}", error);
                    continue;
                },
            };

            Command::UinputCommand(UinputCommand::TextType(text))
        } else if event_name == "spawn" {
            if event_vector.len() != 1 {
                println!("Cannot parse event, not enough items");
                continue;
            }

            let path = match library::read_as_string(
                interpreter,
                event_vector.remove(0),
            ) {
                Ok(string) => string.clone(),
                Err(error) => {
                    println!("Cannot parse event cause:");
                    println!("{}", error);
                    continue;
                },
            };

            Command::Spawn(path)
        } else if event_name == "wait" {
            let milliseconds = match event_vector.remove(0) {
                Value::Integer(ms) => {
                    if ms >= 0 {
                        ms as u64
                    } else {
                        println!(
                            "Expected duration to be not negative, got: {}",
                            ms
                        );
                        continue;
                    }
                },
                v => {
                    println!("Cannot parse event: expected duration, got:");
                    interpreter.print_value(v);
                    continue;
                },
            };

            Command::Wait(milliseconds)
        } else {
            println!("Unknown event type: {}", event_name);
            continue;
        };

        match cmd_sender.send(command) {
            Ok(_) => {},
            Err(_) => {},
        }
    }

    Ok(())
}

impl EventLoop {
    pub fn run_event_loop(
        interpreter: Interpreter,
    ) -> (
        mpsc::Sender<InterpreterCommand>,
        mpsc::Receiver<CommandResult>,
    ) {
        let mut interpreter = interpreter;

        let (command_sender, command_receiver) =
            mpsc::channel::<InterpreterCommand>();

        let (result_sender, result_receiver) = mpsc::channel::<CommandResult>();

        let (cmd_sender, cmd_stopper) = NiaCommandSender::new().start_sending();

        let mut event_listener_v: Option<(
            mpsc::Receiver<Action>,
            mpsc::Sender<()>,
        )> = None;

        thread::spawn(move || {
            let current_time = SystemTime::now();
            let mut time_for_garbage_collection = current_time
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            time_for_garbage_collection +=
                Duration::from_millis(GARBAGE_COLLECTOR_PERIOD);

            loop {
                // execute command that was received with channel
                match command_receiver.try_recv() {
                    Ok(command) => match command {
                        InterpreterCommand::Execution(code) => {
                            let result =
                                interpreter.execute_in_main_environment(&code);

                            let execution_result = match result {
                                Ok(value) => {
                                    match library::value_to_string(
                                        &mut interpreter,
                                        value,
                                    ) {
                                        Ok(string) => {
                                            ExecutionResult::Success(string)
                                        },
                                        Err(error) => {
                                            if error.is_failure() {
                                                ExecutionResult::Failure(
                                                    error.to_string(),
                                                )
                                            } else {
                                                ExecutionResult::Error(
                                                    error.to_string(),
                                                )
                                            }
                                        },
                                    }
                                },
                                Err(error) => {
                                    if error.is_failure() {
                                        ExecutionResult::Failure(
                                            error.to_string(),
                                        )
                                    } else {
                                        ExecutionResult::Error(
                                            error.to_string(),
                                        )
                                    }
                                },
                            };

                            match result_sender.send(
                                CommandResult::ExecutionResult(
                                    execution_result,
                                ),
                            ) {
                                Ok(()) => {},
                                Err(_) => break,
                            }
                        },
                    },
                    Err(TryRecvError::Disconnected) => {
                        break;
                    },
                    Err(TryRecvError::Empty) => {},
                };

                // construct/stop key remapping threads
                if interpreter.is_listening() && event_listener_v.is_none() {
                    let mut event_listener =
                        NiaEventListener::from_interpreter(&mut interpreter);

                    match event_listener.start_listening(cmd_sender.clone()) {
                        Ok(ok) => event_listener_v = Some(ok),
                        Err(error) => break,
                    }
                } else if !interpreter.is_listening()
                    && event_listener_v.is_some()
                {
                    let (_, stopper) = event_listener_v.unwrap();

                    stopper.send(());

                    event_listener_v = None;
                }

                // execute actions bound on key mappings
                if event_listener_v.is_some() {
                    let action_receiver = match &event_listener_v {
                        Some((action_receiver, _)) => action_receiver,
                        _ => unreachable!(),
                    };

                    loop {
                        match action_receiver.try_recv() {
                            Ok(action) => match action {
                                Action::Empty => {},
                                Action::Execute(value) => {
                                    match interpreter.execute_function(value) {
                                        Ok(_) => {},
                                        Err(error) => {
                                            println!("Error happened:");
                                            println!("{}", error);
                                        },
                                    };
                                },
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

                // send events for execution
                send_events(&mut interpreter, &cmd_sender)
                    .expect("Sending events failed.");

                // collect garbage once in 2 minutes
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards.");

                if current_time >= time_for_garbage_collection {
                    time_for_garbage_collection = current_time
                        + Duration::from_millis(GARBAGE_COLLECTOR_PERIOD);

                    let root_environment =
                        interpreter.get_main_environment_id();
                    collect_garbage(&mut interpreter)
                        .expect("Garbage collection failed.");
                } else {
                    thread::sleep(Duration::from_millis(10));
                }
            }

            match event_listener_v {
                Some((_, stopper)) => {
                    stopper.send(());
                },
                _ => {},
            }

            cmd_stopper.send(());
        });

        (command_sender, result_receiver)
    }
}
