pub mod parser;
pub mod interpreter;

//#[macro_use]
extern crate nom;

use std::io::{self, BufRead, Write};

use crate::interpreter::interpreter::Interpreter;

// todo: implement reference counting
// todo: Add better error handling
// todo: Write stdlib
// todo: Implement keyboard listening
// todo: check tests on arithmetic operations
// todo: binary plugins
// todo: ordinary plugins
// todo: file system
// todo: threading
// todo: implement constant checking, and move checking setting nil errors to interpreter itself

fn main() {
    let mut interpreter = Interpreter::new();
    let stdin = io::stdin();

    print!(">> ");
    io::stdout().flush();

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
                io::stdout().flush();
                continue;
            }
        };

        interpreter.print_value(result);
        println!();
        print!(">> ");
        io::stdout().flush();
    }
}
