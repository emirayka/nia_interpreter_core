pub mod parser;
pub mod interpreter;

//#[macro_use]
extern crate nom;

// todo: Add better error handling
// todo: Write stdlib
// todo: Implement keyboard listening
// todo: Get rid of unnecessary .clone()
// todo: Should "(()t t)" be able to be parsed? It works in emacs lisp. From the other hand, "()t" does not.

fn main() {
    let mut interpreter = interpreter::interpreter::Interpreter::new();

    println!("{:?}", interpreter.execute("'test#huest"));
}
