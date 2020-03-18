extern crate rand;
extern crate nom;

pub mod parser;
pub mod interpreter;

mod repl;

// todo: implement reference counting
// todo: Add better error handling
// todo: Write stdlib
// todo: Implement keyboard listening
// todo: binary plugins
// todo: ordinary plugins
// todo: file system
// todo: threading
// todo: implement constant checking, and move checking setting nil errors to interpreter itself

fn main() -> Result<(), std::io::Error> {
    repl::run()
}
