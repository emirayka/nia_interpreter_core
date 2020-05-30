pub mod utils;

pub mod interpreter;
pub mod repl;

pub use interpreter::*;

// todo: Add better error handling
// todo: binary plugins
// todo: file system
// todo: threading
// todo: doitems loop that loops over object key value pairs

fn main() -> std::io::Result<()> {
    repl::run()?;

    Ok(())
}
