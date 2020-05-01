use crate::Value;
use crate::Error;
use crate::Interpreter;

pub type SpecialVariableFunction = fn(&Interpreter) -> Result<Value, Error>;
