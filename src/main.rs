pub mod parser;
pub mod interpreter;

//#[macro_use]
extern crate nom;

// todo: Add macro support. Done, probably. Need tests.
// todo: Add better erroring
// todo: Write special forms
// todo: Write stdlib
// todo: Implement keyboard listening
// todo: Get rid of unnecessary .clone()
// todo: add error tests for parser

fn main() {
    let mut interpreter = interpreter::interpreter::Interpreter::new();

    interpreter.execute("#t").unwrap();
}
