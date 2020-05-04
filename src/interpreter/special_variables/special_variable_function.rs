use crate::Error;
use crate::Interpreter;
use crate::Value;

pub type SpecialVariableFunction = fn(&Interpreter) -> Result<Value, Error>;
