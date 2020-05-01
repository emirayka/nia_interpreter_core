use crate::Value;
use crate::Error;
use crate::Interpreter;

pub fn _super(interpreter: &Interpreter) -> Result<Value, Error> {
    match interpreter.get_this_object() {
        Some(object_id) => {
            match interpreter.get_object_proto(object_id)? {
                Some(proto_object_id) => Ok(proto_object_id.into()),
                None => Error::generic_execution_error("Variable `super' is undefined.")
                    .into()
            }
        }
        None => Error::generic_execution_error("Variable `super' is undefined.")
            .into()
    }
}