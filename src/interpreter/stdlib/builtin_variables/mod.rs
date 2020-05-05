use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

fn define_variable_with_nil(
    interpreter: &mut Interpreter,
    name: &str,
) -> Result<(), Error> {
    let root_environment_id = interpreter.get_root_environment_id();
    let symbol_id = interpreter.intern_symbol_id(name);
    let value = interpreter.intern_nil_symbol_value();

    interpreter.define_variable(root_environment_id, symbol_id, value)?;

    Ok(())
}

fn define_empty_list(
    interpreter: &mut Interpreter,
    name: &str,
) -> Result<(), Error> {
    define_variable_with_nil(interpreter, name)
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    define_empty_list(interpreter, "registered-keyboards")?;
    define_empty_list(interpreter, "global-map")?;
    define_empty_list(interpreter, "modifiers")?;

    define_empty_list(interpreter, "--actions")?;

    Ok(())
}
