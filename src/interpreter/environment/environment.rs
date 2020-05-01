use std::collections::HashMap;

use crate::interpreter::value::SymbolId;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::environment::EnvironmentValueWrapper;
use crate::interpreter::error::Error;


#[derive(Clone)]
pub struct LexicalEnvironment {
    variables: HashMap<SymbolId, EnvironmentValueWrapper>,
    functions: HashMap<SymbolId, EnvironmentValueWrapper>,
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

fn has_value(map: &HashMap<SymbolId, EnvironmentValueWrapper>, symbol_id: SymbolId) -> bool {
    map.contains_key(&symbol_id)
}

fn define_value(
    map: &mut HashMap<SymbolId, EnvironmentValueWrapper>,
    symbol_id: SymbolId,
    environment_value_wrapper: EnvironmentValueWrapper,
) -> Result<(), Error> {
    if !has_value(map, symbol_id) {
        map.insert(symbol_id, environment_value_wrapper);
        Ok(())
    } else {
        Error::generic_execution_error(
            "Cannot define already defined value."
        ).into()
    }
}

fn set_value(
    map: &mut HashMap<SymbolId, EnvironmentValueWrapper>,
    symbol_id: SymbolId,
    value: Value,
) -> Result<(), Error> {
    if has_value(map, symbol_id) {
        map.get_mut(&symbol_id).unwrap().set_value(value)?;
        Ok(())
    } else {
        Error::generic_execution_error(
            "Cannot set value that does not exist."
        ).into()
    }
}

fn lookup_value(
    map: &HashMap<SymbolId, EnvironmentValueWrapper>,
    symbol_id: SymbolId
) -> Result<Option<Value>, Error> {
    match map.get(&symbol_id) {
        Some(value) => Ok(Some(value.get_value()?)),
        _ => Ok(None)
    }
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

    pub fn lookup_variable(&self, symbol_id: SymbolId) -> Result<Option<Value>, Error> {
        lookup_value(&self.variables, symbol_id)
    }

    pub fn lookup_function(&self, symbol_id: SymbolId) -> Result<Option<Value>, Error> {
        lookup_value(&self.functions, symbol_id)
    }

    pub fn define_variable(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        define_value(
            &mut self.variables,
            symbol_id,
            EnvironmentValueWrapper::new(value)
        ).map_err(|err| Error::generic_execution_error_caused(
            "Cannot define variable.",
            err,
        ))
    }

    pub fn define_const_variable(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        define_value(
            &mut self.variables,
            symbol_id,
        EnvironmentValueWrapper::new_const(value)
        ).map_err(|err| Error::generic_execution_error_caused(
            "Cannot define variable.",
            err,
        ))
    }

    pub fn define_function(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        define_value(
            &mut self.functions,
            symbol_id,
            EnvironmentValueWrapper::new(value)
        ).map_err(|err| Error::generic_execution_error_caused(
            "Cannot define function.",
            err,
        ))
    }

    pub fn define_const_function(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        define_value(
            &mut self.functions,
            symbol_id,
        EnvironmentValueWrapper::new_const(value)
        ).map_err(|err| Error::generic_execution_error_caused(
            "Cannot define function.",
            err,
        ))
    }

    pub fn set_variable(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        set_value(
            &mut self.variables,
            symbol_id,
            value
        ).map_err(|err| Error::generic_execution_error_caused(
            "Cannot set variable.",
            err
        ))
    }

    pub fn set_function(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        set_value(
            &mut self.functions,
            symbol_id,
            value
        ).map_err(|err| Error::generic_execution_error_caused(
            "Cannot set function.",
            err
        ))
    }

    pub fn get_gc_items(&self) -> Vec<Value> {
        let mut result = self.variables
            .values()
            .into_iter()
            .map(|value| value.to_value())
            .collect::<Vec<Value>>();

        result.extend(self.functions
            .values()
            .into_iter()
            .map(|value| value.to_value()));

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
        assert_eq!(Some(Value::Integer(1)), env.lookup_variable(key).unwrap());

        assert!(!env.has_function(key));
        env.define_function(key, Value::Integer(1)).unwrap();
        assert!(env.has_function(key));
        assert_eq!(Some(Value::Integer(1)), env.lookup_function(key).unwrap());
    }

    #[test]
    fn able_to_change_bindings() {
        let mut env = LexicalEnvironment::new();
        let key = SymbolId::new(0);

        env.define_variable(key, Value::Integer(1)).unwrap();
        env.define_function(key, Value::Integer(1)).unwrap();

        env.set_variable(key, Value::Integer(2)).unwrap();
        env.set_function(key, Value::Integer(2)).unwrap();

        assert_eq!(Some(Value::Integer(2)), env.lookup_variable(key).unwrap());
        assert_eq!(Some(Value::Integer(2)), env.lookup_function(key).unwrap());
    }

    #[test]
    fn cannot_set_not_defined_value() {
        let mut env = LexicalEnvironment::new();
        let key = SymbolId::new(0);

        assert!(env.set_variable(key, Value::Integer(2)).is_err());
        assert!(env.set_function(key, Value::Integer(2)).is_err());
    }

    #[test]
    fn cannot_define_value_twice() {
        let mut env = LexicalEnvironment::new();
        let key = SymbolId::new(0);

        env.define_variable(key, Value::Integer(1)).unwrap();
        assert!(env.define_variable(key, Value::Integer(1)).is_err());

        env.define_function(key, Value::Integer(1)).unwrap();
        assert!(env.define_function(key, Value::Integer(1)).is_err());
    }

    #[test]
    fn able_to_make_parent_relationship() {
        let mut env = LexicalEnvironment::new();
        let id = EnvironmentId::new(1);

        assert_eq!(None, env.get_parent());

        env.set_parent(id);

        assert_eq!(Some(id), env.get_parent());
    }

    #[test]
    fn able_to_define_constant_variables() {
        let mut env = LexicalEnvironment::new();
        let symbol = SymbolId::new(0);
        let value = Value::Integer(1);

        env.define_const_variable(symbol, value);

        assert_eq!(Some(value), env.lookup_variable(symbol).unwrap());
        assert!(env.set_variable(symbol, Value::Integer(2)).is_err());
    }

    #[test]
    fn able_to_define_constant_functions() {
        let mut env = LexicalEnvironment::new();
        let symbol = SymbolId::new(0);
        let value = Value::Integer(1);

        env.define_const_function(symbol, value);

        assert_eq!(Some(value), env.lookup_function(symbol).unwrap());
        assert!(env.set_function(symbol, Value::Integer(2)).is_err());
    }
}
