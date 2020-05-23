use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

pub const DEFINED_DEVICES_ROOT_VARIABLE_NAME: &'static str =
    "nia-defined-devices";
pub const DEFINED_MODIFIERS_ROOT_VARIABLE_NAME: &'static str =
    "nia-defined-modifiers";
pub const DEFINED_ACTIONS_ROOT_VARIABLE_NAME: &'static str =
    "nia-defined-actions";
pub const GLOBAL_MAP_ROOT_VARIABLE_NAME: &'static str = "nia-global-map";
pub const PRIMITIVE_ACTIONS_VARIABLE_NAME: &'static str =
    "nia-primitive-actions";

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
    define_empty_list(interpreter, DEFINED_DEVICES_ROOT_VARIABLE_NAME)?;
    define_empty_list(interpreter, DEFINED_MODIFIERS_ROOT_VARIABLE_NAME)?;
    define_empty_list(interpreter, DEFINED_ACTIONS_ROOT_VARIABLE_NAME)?;
    define_empty_list(interpreter, GLOBAL_MAP_ROOT_VARIABLE_NAME)?;

    define_empty_list(interpreter, PRIMITIVE_ACTIONS_VARIABLE_NAME)?;

    Ok(())
}
