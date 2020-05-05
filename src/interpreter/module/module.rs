use std::collections::HashMap;

use crate::EnvironmentId;
use crate::Interpreter;
use crate::Value;
use crate::{Error, SymbolId};

#[derive(Clone, Debug)]
pub struct Module {
    path: String,
    environment_id: EnvironmentId,
    exports: HashMap<SymbolId, Value>,
    default_export: Option<Value>,
}

impl Module {
    pub fn new(path: String, environment_id: EnvironmentId) -> Module {
        Module {
            path,
            environment_id,
            exports: HashMap::new(),
            default_export: None,
        }
    }

    pub fn new_root_module(environment_id: EnvironmentId) -> Module {
        Module::new(String::from(""), environment_id)
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_environment_id(&self) -> EnvironmentId {
        self.environment_id
    }

    pub fn get_exports(&self) -> &HashMap<SymbolId, Value> {
        &self.exports
    }

    pub fn get_export(&self, symbol_id: SymbolId) -> Option<Value> {
        self.exports.get(&symbol_id).map(|v| *v)
    }

    pub fn get_default_export(&self) -> Option<Value> {
        self.default_export
    }

    pub fn add_export(
        &mut self,
        symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        if self.exports.contains_key(&symbol_id) {
            Error::generic_execution_error("Named export was already defined.")
                .into()
        } else {
            self.exports.insert(symbol_id, value);
            Ok(())
        }
    }

    pub fn add_default_export(&mut self, value: Value) -> Result<(), Error> {
        match self.default_export {
            Some(_) => {
                return Error::generic_execution_error(
                    "Default export was already defined.",
                )
                .into();
            },
            None => {
                self.default_export = Some(value);
            },
        }

        Ok(())
    }
}
