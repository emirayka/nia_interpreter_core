use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

pub fn fun(interpreter: &mut Interpreter, environment: EnvironmentId, values: Vec<Value>) -> Result<Value, Error> {
}
