use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::function::arguments::Arguments;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroFunction {
    environment: EnvironmentId,
    arguments: Arguments,
    code: Vec<Value>,
}

impl MacroFunction {
    pub fn new(environment: EnvironmentId, arguments: Arguments, code: Vec<Value>) -> MacroFunction {
        MacroFunction {
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

    pub fn get_gc_items(&self) -> Option<Vec<Value>> {
        let mut result = self.code.clone();

        result.extend(self.arguments.get_gc_items());

        Some(result)
    }

    pub fn get_gc_environment(&self) -> Option<EnvironmentId> {
        Some(self.environment)
    }
}
