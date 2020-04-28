use std::collections::HashMap;

use crate::interpreter::value::SymbolId;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;

pub const VALUE_WRAPPER_FLAG_SETTABLE: u8 = 0x1;
pub const VALUE_WRAPPER_FLAG_GETTABLE: u8 = 0x2;
pub const VALUE_WRAPPER_FLAG_CONFIGURABLE: u8 = 0x4;

pub const VALUE_WRAPPER_FLAG_DEFAULT: u8 =
    VALUE_WRAPPER_FLAG_SETTABLE | VALUE_WRAPPER_FLAG_GETTABLE | VALUE_WRAPPER_FLAG_CONFIGURABLE;

pub const VALUE_WRAPPER_FLAG_CONST: u8 =
    VALUE_WRAPPER_FLAG_GETTABLE;

#[derive(Copy, Clone, Debug)]
pub struct EnvironmentValueWrapper {
    value: Value,
    flags: u8,
}

impl EnvironmentValueWrapper {
    pub fn with_flags(value: Value, flags: u8) -> EnvironmentValueWrapper {
        EnvironmentValueWrapper {
            value,
            flags,
        }
    }

    pub fn is_settable(&self) -> bool {
        self.flags & VALUE_WRAPPER_FLAG_SETTABLE != 0
    }

    pub fn is_gettable(&self) -> bool {
        self.flags & VALUE_WRAPPER_FLAG_GETTABLE != 0
    }

    pub fn is_configurable(&self) -> bool {
        self.flags & VALUE_WRAPPER_FLAG_GETTABLE != 0
    }

    pub fn check_is_gettable(&self) -> Result<(), Error> {
        if self.is_gettable() {
            Ok(())
        } else {
            Error::generic_execution_error(
                "Cannot intern not internable item."
            ).into_result()
        }
    }

    pub fn check_is_settable(&self) -> Result<(), Error> {
        if self.is_settable() {
            Ok(())
        } else {
            Error::generic_execution_error(
                "Cannot change const item."
            ).into_result()
        }
    }

    pub fn check_is_configurable(&self) -> Result<(), Error> {
        if self.is_configurable() {
            Ok(())
        } else {
            Error::generic_execution_error(
                "Cannot configure not configurable item."
            ).into_result()
        }
    }

    pub fn set_flags(&mut self, flags: u8) -> Result<(), Error> {
        self.check_is_configurable()?;
        self.flags = flags;

        Ok(())
    }

    pub fn set_value(&mut self, value: Value) -> Result<(), Error> {
        self.check_is_settable()?;
        self.value = value;

        Ok(())
    }

    pub fn get_value(&self) -> Result<Value, Error>{
        self.check_is_gettable()?;

        Ok(self.value)
    }

    // must be used by LexicalEnvironment only
    pub fn to_value(&self) -> Value {
        self.value
    }
}

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
    value: Value,
    flags: u8
) -> Result<(), Error> {
    if !has_value(map, symbol_id) {
        map.insert(symbol_id, EnvironmentValueWrapper::with_flags(value, flags));
        Ok(())
    } else {
        Error::generic_execution_error(
            "Cannot define already defined value."
        ).into_result()
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
            "Cannot set value that does not exist"
        ).into_result()
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
            value,
            VALUE_WRAPPER_FLAG_DEFAULT
        ).map_err(|err| Error::generic_execution_error_caused(
            "Cannot define variable.",
            err,
        ))
    }

    pub fn define_const_variable(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        define_value(
            &mut self.variables,
            symbol_id,
            value,
            VALUE_WRAPPER_FLAG_CONST
        ).map_err(|err| Error::generic_execution_error_caused(
            "Cannot define variable.",
            err,
        ))
    }

    pub fn define_function(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        define_value(
            &mut self.functions,
            symbol_id,
            value,
            VALUE_WRAPPER_FLAG_DEFAULT
        ).map_err(|err| Error::generic_execution_error_caused(
            "Cannot define function.",
            err,
        ))
    }

    pub fn define_const_function(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        define_value(
            &mut self.functions,
            symbol_id,
            value,
            VALUE_WRAPPER_FLAG_CONST
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
    fn makes_updates_bindings() {
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
        let mut env2 = LexicalEnvironment::new();
        let id1 = EnvironmentId::new(1);

        assert_eq!(None, env2.get_parent());

        env2.set_parent(id1);

        assert_eq!(Some(id1), env2.get_parent());
    }
}
