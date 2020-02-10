use crate::interpreter::value::Value;
use crate::interpreter::environment::environment_arena::EnvironmentId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroFunction {
    environment: EnvironmentId,
    argument_names: Vec<String>,
    code: Vec<Value>,
}

impl MacroFunction {
    pub fn new(environment: EnvironmentId, argument_names: Vec<String>, code: Vec<Value>) -> MacroFunction {
        MacroFunction {
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
