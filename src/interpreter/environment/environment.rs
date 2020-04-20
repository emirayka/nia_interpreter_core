use std::collections::HashMap;

use crate::interpreter::symbol::SymbolId;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;

#[derive(Clone)]
pub struct LexicalEnvironment {
    variables: HashMap<SymbolId, Value>,
    functions: HashMap<SymbolId, Value>,
    parent: Option<EnvironmentId>,
}

impl LexicalEnvironment {
    pub fn new() -> LexicalEnvironment {
        LexicalEnvironment {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
        }
    }
}

fn lookup_value(map: &HashMap<SymbolId, Value>, symbol_id: SymbolId) -> Option<Value> {
    match map.get(&symbol_id) {
        Some(value) => Some(*value),
        _ => None
    }
}

fn set_value(map: &mut HashMap<SymbolId, Value>, symbol_id: SymbolId, value: Value) {
    match map.get_mut(&symbol_id) {
        Some(value_ref) => {
            *value_ref = value;
        },
        None => {
            map.insert(symbol_id, value);
        }
    };
}

fn has_value(map: &HashMap<SymbolId, Value>, symbol_id: SymbolId) -> bool {
    map.contains_key(&symbol_id)
}

impl LexicalEnvironment {
    pub fn get_parent(&self) -> Option<EnvironmentId> {
        self.parent
    }

    pub fn set_parent(&mut self, parent_id: EnvironmentId) {
        self.parent = Some(parent_id)
    }

    pub fn has_variable(&self, symbol_id: SymbolId) -> bool {
        has_value(&self.variables, symbol_id)
    }

    pub fn has_function(&self, symbol_id: SymbolId) -> bool {
        has_value(&self.functions, symbol_id)
    }

    pub fn lookup_variable(&self, symbol_id: SymbolId) -> Option<Value> {
        let result = lookup_value(&self.variables, symbol_id);

        if let Some(found_value) = result {
            Some(found_value)
        } else {
            None
        }
    }

    pub fn lookup_function(&self, symbol_id: SymbolId) -> Option<Value> {
        let result = lookup_value(&self.functions, symbol_id);

        if let Some(found_value) = result {
            Some(found_value)
        } else {
            None
        }
    }

    pub fn define_variable(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        if !self.has_variable(symbol_id) {
            set_value(&mut self.variables, symbol_id, value);
            Ok(())
        } else {
            Error::generic_execution_error(
                "Cannot define the same variable twice."
            ).into_result()
        }
    }

    pub fn define_function(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        if !self.has_function(symbol_id) {
            set_value(&mut self.functions, symbol_id, value);
            Ok(())
        } else {
            Error::generic_execution_error(
                "Cannot define the same function twice."
            ).into_result()
        }
    }

    pub fn set_variable(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        if self.has_variable(symbol_id) {
            set_value(&mut self.variables, symbol_id, value);
            Ok(())
        } else {
            Error::generic_execution_error(
                "Cannot set value of not defined variable."
            ).into_result()
        }
    }

    pub fn set_function(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        if self.has_function(symbol_id) {
            set_value(&mut self.functions, symbol_id, value);
            Ok(())
        } else {
            Error::generic_execution_error(
                "Cannot set value of not defined function."
            ).into_result()
        }
    }

    pub fn get_gc_items(&self) -> Vec<Value> {
        let mut result = self.variables
            .values()
            .into_iter()
            .map(|value| *value)
            .collect::<Vec<Value>>();

        result.extend(self.functions
            .values()
            .into_iter()
            .map(|value| *value));

        result.extend(self.variables
            .keys()
            .into_iter()
            .map(|symbol_id| Value::Symbol(*symbol_id)));

        result.extend(self.functions
            .keys()
            .into_iter()
            .map(|symbol_id| Value::Symbol(*symbol_id)));

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_new_bindings() {
        let mut env = LexicalEnvironment::new();
        let key = SymbolId::new(0);

        assert!(!env.has_variable(key));
        env.define_variable(key, Value::Integer(1)).unwrap();
        assert!(env.has_variable(key));
        assert_eq!(Value::Integer(1), env.lookup_variable(key).unwrap());

        assert!(!env.has_function(key));
        env.define_function(key, Value::Integer(1)).unwrap();
        assert!(env.has_function(key));
        assert_eq!(Value::Integer(1), env.lookup_function(key).unwrap());
    }

    #[test]
    fn makes_updates_bindings() {
        let mut env = LexicalEnvironment::new();
        let key = SymbolId::new(0);

        env.define_variable(key, Value::Integer(1)).unwrap();
        env.define_function(key, Value::Integer(1)).unwrap();

        env.set_variable(key, Value::Integer(2)).unwrap();
        env.set_function(key, Value::Integer(2)).unwrap();

        assert_eq!(Value::Integer(2), env.lookup_variable(key).unwrap());
        assert_eq!(Value::Integer(2), env.lookup_function(key).unwrap());
    }

    #[test]
    fn cannot_set_to_not_defined_variable() {
        let mut env = LexicalEnvironment::new();
        let key = SymbolId::new(0);

        assert!(env.set_variable(key, Value::Integer(2)).is_err());
    }

    #[test]
    fn cannot_set_to_not_defined_function() {
        let mut env = LexicalEnvironment::new();
        let key = SymbolId::new(0);

       assert!(env.set_function(key, Value::Integer(2)).is_err()) ;
    }

    #[test]
    fn cannot_define_variable_twice() {
        let mut env = LexicalEnvironment::new();
        let key = SymbolId::new(0);

        env.define_variable(key, Value::Integer(1)).unwrap();
        assert!(env.define_variable(key, Value::Integer(1)).is_err());
    }

    #[test]
    fn cannot_define_function_twice() {
        let mut env = LexicalEnvironment::new();
        let key = SymbolId::new(0);

        env.define_function(key, Value::Integer(1)).unwrap();
        assert!(env.define_function(key, Value::Integer(1)).is_err());
    }

    #[test]
    fn able_to_make_parent_relationship() {
        let mut env2 = LexicalEnvironment::new();
        let id1 = EnvironmentId::new(1);

        assert_eq!(None, env2.get_parent());

        env2.set_parent(id1);

        assert_eq!(Some(id1), env2.get_parent());
    }
}
