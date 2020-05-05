use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::Path;

use crate::interpreter::ErrorKind;
use crate::interpreter::Interpreter;
use crate::{CommandResult, EventLoop, InterpreterCommand};

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
    let history_file = get_history_file_path();

    let mut interpreter = Interpreter::new();

    let (sender, receiver) = EventLoop::run_event_loop(interpreter);

    let mut rl = Editor::<()>::new();

    if let Some(history) = &history_file {
        rl.load_history(history);
    } else {
        println!("History file can't be constructed.");
    }

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                sender.send(InterpreterCommand::Execution(line));

                let result = match receiver.recv() {
                    Ok(CommandResult::ExecutionResult(result)) => result,
                    Err(_) => break,
                };

                println!("{}", result);
            },
            Err(ReadlineError::Interrupted) => {
                // break;
            },
            Err(ReadlineError::Eof) => {
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            },
        }
    }

    if let Some(history) = &history_file {
        rl.save_history(history)
            .expect(&format!("Failure saving history at: {}", history));
    }

    Ok(())
}
