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