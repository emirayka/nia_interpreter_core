use std::fmt;

use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub type BuiltinFunctionType = fn(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    argument_values: Vec<Value>,
) -> Result<Value, Error>;

#[derive(Clone)]
pub struct BuiltinFunction {
    func: BuiltinFunctionType,
}

impl fmt::Debug for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<built-in function>")
    }
}

impl PartialEq for BuiltinFunction {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

impl Eq for BuiltinFunction {}

impl BuiltinFunction {
    pub fn new(func: BuiltinFunctionType) -> BuiltinFunction {
        BuiltinFunction { func }
    }

    pub fn get_func(&self) -> &BuiltinFunctionType {
        &self.func
    }

    pub fn get_gc_items(&self) -> Option<Vec<Value>> {
        None
    }

    pub fn get_gc_environment(&self) -> Option<EnvironmentId> {
        None
    }
}
