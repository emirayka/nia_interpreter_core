pub mod parser;
pub mod interpreter;

//#[macro_use]
extern crate nom;

// todo: Add better error handling
// todo: Write stdlib
// todo: Implement keyboard listening
// todo: Should "(()t t)" be able to be parsed? It works in emacs lisp. From the other hand, "()t" does not.
// todo: check test on arithmetic operations
// todo: implement reference counting
// todo: add clojure-like "#(+ %1 %2)" -> "(function (lambda (%1 %2) (+ %1 %2)))" function definition
// todo: add variadic args
// todo: add key word args
// todo: add checking of the same argument name
// todo: binary plugins
// todo: ordinary plugins
// todo: file system
// todo: implement constant checking, and move checking setting nil errors to interpreter itself

fn main() {
    let mut interpreter = interpreter::interpreter::Interpreter::new();

    println!("{:?}", interpreter.execute("{}"));
}
