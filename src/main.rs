pub mod parser;
pub mod interpreter;

//#[macro_use]
extern crate nom;

// todo: Add better error handling
// todo: Write stdlib
// todo: Implement keyboard listening
// todo: Get rid of unnecessary .clone()
// todo: Should "(()t t)" be able to be parsed? It works in emacs lisp. From the other hand, "()t" does not.
// todo: check test on arithmetic operations
// todo: major problem is freeing memory in arenas
// todo: add shortcut "(object:func 1 2)" for method invocations
// todo: add shortcut "(+ object:v1 object:v2)" for value lookups
// todo: add clojure-like "#(+ %1 %2)" -> "(function (lambda (%1 %2) (+ %1 %2)))" function definition
// todo: add variadic args
// todo: add key word args
// todo: add checking of the same argument name
// todo: binary plugins
// todo: ordinary plugins
// todo: file system

fn main() {
    let mut interpreter = interpreter::interpreter::Interpreter::new();

    println!("{:?}", interpreter.execute("{}"));
}
