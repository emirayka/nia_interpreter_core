use std::fmt;

use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub type SpecialFormFunctionType = fn(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    argument_values: Vec<Value>,
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
        self as *const _ == other as *const _
    }
}

impl Eq for SpecialFormFunction {}

impl SpecialFormFunction {
    pub fn new(func: SpecialFormFunctionType) -> SpecialFormFunction {
        SpecialFormFunction { func }
    }

    pub fn get_func(&self) -> &SpecialFormFunctionType {
        &self.func
    }

    pub fn get_gc_items(&self) -> Option<Vec<Value>> {
        None
    }

    pub fn get_gc_environment(&self) -> Option<EnvironmentId> {
        None
    }
}
