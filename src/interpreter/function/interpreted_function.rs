use crate::interpreter::value::Value;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::function::arguments::Arguments;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterpretedFunction {
    environment: EnvironmentId,
    arguments: Arguments,
    code: Vec<Value>,
}

impl InterpretedFunction {
    pub fn new(environment: EnvironmentId, arguments: Arguments, code: Vec<Value>) -> InterpretedFunction {
        InterpretedFunction {
            environment,
            arguments,
            code
        }
    }

    pub fn get_environment(&self) -> EnvironmentId {
        self.environment
    }

    pub fn get_arguments(&self) -> &Arguments {
        &self.arguments
    }

    pub fn get_code(&self) -> &Vec<Value> {
        &self.code
    }
}
