use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::interpreter::Interpreter;
use crate::EventLoop;
use crate::NiaExecuteCodeCommand;
use crate::NiaExecuteCodeCommandResult;
use crate::NiaInterpreterCommand;
use crate::NiaInterpreterCommandResult;

const HISTORY_FILE_NAME: &'static str = ".nia-interpreter.history";

fn get_history_file_path() -> Option<String> {
    match dirs::home_dir() {
        Some(dir) => match dir.as_path().join(HISTORY_FILE_NAME).to_str() {
            Some(s) => Some(s.to_string()),
            _ => None,
        },
        _ => None,
    }
}

pub fn run() -> Result<(), std::io::Error> {
    let interpreter = Interpreter::with_default_config();
    let history_file = get_history_file_path();

    let event_loop_handle = EventLoop::run_event_loop(interpreter);

    let mut rl = Editor::<()>::new();

    if let Some(history) = &history_file {
        rl.load_history(history)
            .expect("Failure reading history file.");
    } else {
        println!("History file can't be constructed.");
    }

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                event_loop_handle.send_command(
                    NiaInterpreterCommand::ExecuteCode(
                        NiaExecuteCodeCommand::new(line),
                    ),
                );

                let result = match event_loop_handle.receive_result() {
                    Ok(result) => result,
                    Err(_) => break,
                };

                if let NiaInterpreterCommandResult::ExecuteCode(result) = result
                {
                    match result {
                        NiaExecuteCodeCommandResult::Success(success) => {
                            println!("{}", success)
                        }
                        NiaExecuteCodeCommandResult::Error(error) => {
                            println!("Error: {}", error)
                        }
                        NiaExecuteCodeCommandResult::Failure(failure) => {
                            println!("Failure: {}", failure)
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    if let Some(history) = &history_file {
        rl.save_history(history)
            .expect(&format!("Failure saving history at: {}", history));
    }

    Ok(())
}
