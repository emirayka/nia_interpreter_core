pub mod utils;

pub mod interpreter;
pub mod repl;

pub use interpreter::*;

// todo: Add better error handling
// todo: binary plugins
// todo: file system
// todo: threading

fn main() -> std::io::Result<()> {
    repl::run()?;
    // let code = "(define-function defm (function (macro (name #rest params) (list:new 'define-function name (list:new 'function (cons:new 'macro params))))))";
    // let mut interpreter = Interpreter::new();

    Ok(())
}
