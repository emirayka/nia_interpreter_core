use std::fmt;

use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

pub type SpecialFormFunctionType = fn(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    argument_values: Vec<Value>
) -> Result<Value, Error>;

#[derive(Clone)]
pub struct SpecialFormFunction {
    func: SpecialFormFunctionType,
}

impl fmt::Debug for SpecialFormFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "special form")
    }
}

impl PartialEq for SpecialFormFunction {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _ // todo: maybe this is incorrect
    }
}

impl Eq for SpecialFormFunction {}

impl SpecialFormFunction {
    pub fn new(func: SpecialFormFunctionType) -> SpecialFormFunction {
        SpecialFormFunction {
            func
        }
    }

    pub fn get_func(&self) -> &SpecialFormFunctionType{
        &self.func
    }
}
