use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::interpreter::Interpreter;
use std::fmt::Debug;
use nom::lib::std::fmt::{Formatter, Error};

#[derive(Debug, Clone)]
pub struct InterpretedFunction {
    environment: EnvironmentId,
    argument_names: Vec<String>,
    code: Vec<Value>,
}

impl InterpretedFunction {
    pub fn new(environment: EnvironmentId, argument_names: Vec<String>, code: Vec<Value>) -> InterpretedFunction {
        InterpretedFunction {
            environment,
            argument_names,
            code
        }
    }

    pub fn get_environment(&self) -> EnvironmentId {
        self.environment
    }

    pub fn get_argument_names(&self) -> &Vec<String> {
        &self.argument_names
    }

    pub fn get_code(&self) -> &Vec<Value> {
        &self.code
    }
}

type BuiltInFunctionType = fn(argument_values: Vec<Value>) -> Result<Value, ()>;

#[derive(Clone)]
pub struct BuiltInFunction {
    func: BuiltInFunctionType,
}

impl Debug for BuiltInFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "<built-in function>")
    }
}


impl BuiltInFunction {
    pub fn new(func: BuiltInFunctionType) -> BuiltInFunction {
        BuiltInFunction {
            func
        }
    }

    pub fn get_func(&self) -> &BuiltInFunctionType {
        &self.func
    }
}

#[derive(Debug, Clone)]
pub enum Function {
    Interpreted(InterpretedFunction),
    BuiltIn(BuiltInFunction),
}
