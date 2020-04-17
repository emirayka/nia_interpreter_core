use std::path::Path;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::ErrorKind;

const HISTORY_FILE_NAME: &'static str = ".nia-interpreter.history";

fn get_history_file_path() -> Option<String> {
    match dirs::home_dir() {
        Some(dir) => {
            match dir.as_path().join(HISTORY_FILE_NAME).to_str() {
                Some(s) => Some(s.to_string()),
                _ => None
            }
        },
        _ => {
            None
        }
    }
}

pub fn run() -> Result<(), std::io::Error> {
    let history_file = get_history_file_path();

    let mut interpreter = Interpreter::new();

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

                let value = interpreter.execute(&line);

                match value {
                    Ok(value) => {
                        interpreter.print_value(value);
                    },
                    Err(err) =>  {
                        err.describe();

                        if err.get_error_kind() == ErrorKind::Failure {
                            println!(
                                "Failure occured. Interpreter will be terminated now.\
Likely it's a bug. Please open an issue."
                            );
                            break;
                        }
                    }
                }
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
            }
        }
    }

    if let Some(history) = &history_file {
        rl.save_history(history)
            .expect(&format!("Failure saving history at: {}", history));
    }

    Ok(())
}