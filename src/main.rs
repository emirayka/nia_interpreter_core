use std::collections::hash_map::DefaultHasher;
pub mod utils;

pub mod interpreter;
pub mod repl;

pub use interpreter::*;
use nia_state_machine::{StateMachine, StateMachineResult};

fn main() -> std::io::Result<()> {
    repl::run()?;

    Ok(())
}
