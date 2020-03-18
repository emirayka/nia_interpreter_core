use std::io::{self, BufRead, Write};
use crate::interpreter::interpreter::Interpreter;

pub fn run() -> Result<(), std::io::Error> {
    let mut interpreter = Interpreter::new();
    let stdin = io::stdin();

    print!(">> ");
    io::stdout().flush()?;

    for line in stdin.lock().lines() {
        let string = match line {
            Ok(string) => string,
            _ => {
                println!("Error while reading input");
                break;
            }
        };

        let result = match interpreter.execute(&string) {
            Ok(value) => value,
            Err(error) => {
                println!("Error occured:");
                error.describe();

                print!(">> ");
                io::stdout().flush()?;
                continue;
            }
        };

        interpreter.print_value(result);
        println!();
        print!(">> ");
        io::stdout().flush()?;
    }

    Ok(())
}