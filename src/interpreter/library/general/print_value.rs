use crate::Value;
use crate::{Error, Interpreter};

use crate::library;

pub fn print_value(
    interpreter: &Interpreter,
    value: Value,
) -> Result<(), Error> {
    let string = library::value_to_string(interpreter, value)?;

    println!("{}", string);

    Ok(())
}
