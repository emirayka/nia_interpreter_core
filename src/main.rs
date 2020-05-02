extern crate either;
extern crate dirs;
extern crate rand;
extern crate nom;

extern crate nia_state_machine;
extern crate nia_events;

pub mod parser;
pub mod repl;
pub mod interpreter;

pub use interpreter::*;

// todo: Add better error handling
// todo: binary plugins
// todo: ordinary plugins
// todo: file system
// todo: threading
// todo: implement constant checking, and move checking setting nil errors to interpreter itself

fn main() -> std::io::Result<()> {
    repl::run()?;

    Ok(())
}

