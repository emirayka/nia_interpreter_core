use crate::Error;
use crate::Interpreter;

mod builtin_functions;
mod builtin_objects;
mod builtin_variables;
mod special_forms;

mod core;

pub fn infect_stdlib(interpreter: &mut Interpreter) -> Result<(), Error> {
    special_forms::infect(interpreter)?;
    builtin_functions::infect(interpreter)?;
    builtin_objects::infect(interpreter)?;
    builtin_variables::infect(interpreter)?;

    core::infect(interpreter)?;

    Ok(())
}

pub use builtin_variables::{
    DEFINED_ACTIONS_ROOT_VARIABLE_NAME, DEFINED_DEVICES_ROOT_VARIABLE_NAME,
    DEFINED_MODIFIERS_ROOT_VARIABLE_NAME, GLOBAL_MAP_ROOT_VARIABLE_NAME,
    PRIMITIVE_ACTIONS_VARIABLE_NAME,
};
