pub mod utils;

pub mod interpreter;
pub mod parser;
pub mod repl;

pub use interpreter::*;

// todo: Add better error handling
// todo: binary plugins
// todo: file system
// todo: threading

fn main() -> std::io::Result<()> {
    repl::run()?;

    Ok(())
}
