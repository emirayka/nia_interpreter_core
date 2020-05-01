use crate::Value;
use crate::Error;
use crate::Interpreter;

pub fn _this(interpreter: &Interpreter) -> Result<Value, Error> {
    match interpreter.get_this_object() {
        Some(object_id) => Ok(object_id.into()),
        None => Error::generic_execution_error("Variable `this' is undefined.")
            .into()
    }
}
