use std::collections::HashMap;

use crate::interpreter::symbol::Symbol;
use crate::interpreter::value::Value;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub struct LexicalEnvironment {
    variables: HashMap<Symbol, Value>,
    functions: HashMap<Symbol, Value>,
    parent: Option<EnvironmentId>,
    children: Vec<EnvironmentId>,
}

impl LexicalEnvironment {
    pub fn new() -> LexicalEnvironment {
        LexicalEnvironment {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
            children: Vec::new(),
        }
    }
}

fn lookup_value<'a, 'b>(map: &'a HashMap<Symbol, Value>, symbol: &'b Symbol) -> Option<&'a Value> {
    map.get(symbol)
}

fn set_value(map: &mut HashMap<Symbol, Value>, symbol: &Symbol, value: Value) {
    match map.get_mut(symbol) {
        Some(value_ref) => {
            *value_ref = value;
        },
        None => {
            map.insert(symbol.clone(), value);
        }
    };
}

fn has_value(map: &HashMap<Symbol, Value>, symbol: &Symbol) -> bool {
    map.contains_key(symbol)
}

impl LexicalEnvironment {
    pub fn get_parent(&self) -> Option<EnvironmentId> {
        self.parent
    }

    pub fn set_parent(&mut self, parent_id: EnvironmentId) {
        self.parent = Some(parent_id)
    }

    pub fn add_child(&mut self, child_id: EnvironmentId) {
        self.children.push(child_id)
    }

    pub fn has_variable(&self, symbol: &Symbol) -> bool {
        has_value(&self.variables, symbol)
    }

    pub fn has_function(&self, symbol: &Symbol) -> bool {
        has_value(&self.functions, symbol)
    }

    pub fn lookup_variable(&self, symbol: &Symbol) -> Option<&Value> {
        let result = lookup_value(&self.variables, symbol);

        if let Some(found_value) = result {
            Some(found_value)
        } else {
            None
        }
    }

    pub fn lookup_function(&self, symbol: &Symbol) -> Option<&Value> {
        let result = lookup_value(&self.functions, symbol);

        if let Some(found_value) = result {
            Some(found_value)
        } else {
            None
        }
    }

    pub fn define_variable(&mut self, symbol: &Symbol, value: Value) -> Result<(), ()> {
        if !self.has_variable(symbol) {
            set_value(&mut self.variables, symbol, value);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn define_function(&mut self, symbol: &Symbol, value: Value) -> Result<(), ()> {
        if !self.has_function(symbol) {
            set_value(&mut self.functions, symbol, value);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn set_variable(&mut self, symbol: &Symbol, value: Value) -> Result<(), ()> {
        if self.has_variable(symbol) {
            set_value(&mut self.variables, symbol, value);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn set_function(&mut self, symbol: &Symbol, value: Value) -> Result<(), ()> {
        if self.has_function(symbol) {
            set_value(&mut self.functions, symbol, value);
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::symbol::SymbolArena;

    fn new_symbol(name: &str) -> Symbol {
        let mut symbol_arena = SymbolArena::new();

        symbol_arena.intern(name)
    }

    #[test]
    fn test_makes_new_bindings() {
        let mut env = LexicalEnvironment::new();
        let key = new_symbol("test");

        assert!(!env.has_variable(&key));
        env.define_variable(&key, Value::Integer(1)).unwrap();
        assert!(env.has_variable(&key));
        assert_eq!(&Value::Integer(1), env.lookup_variable(&key).unwrap());

        assert!(!env.has_function(&key));
        env.define_function(&key, Value::Integer(1)).unwrap();
        assert!(env.has_function(&key));
        assert_eq!(&Value::Integer(1), env.lookup_function(&key).unwrap());
    }

    #[test]
    fn test_makes_updates_bindings() {
        let mut env = LexicalEnvironment::new();
        let key = new_symbol("key");

        env.define_variable(&key, Value::Integer(1)).unwrap();
        env.define_function(&key, Value::Integer(1)).unwrap();

        env.set_variable(&key, Value::Integer(2)).unwrap();
        env.set_function(&key, Value::Integer(2)).unwrap();

        assert_eq!(&Value::Integer(2), env.lookup_variable(&key).unwrap());
        assert_eq!(&Value::Integer(2), env.lookup_function(&key).unwrap());
    }

    #[test]
    fn test_cannot_set_to_not_defined_variable() {
        let mut env = LexicalEnvironment::new();
        let key = new_symbol("key");

        assert!(env.set_variable(&key, Value::Integer(2)).is_err());
    }

    #[test]
    fn test_cannot_set_to_not_defined_function() {
        let mut env = LexicalEnvironment::new();
        let key = new_symbol("key");

       assert!(env.set_function(&key, Value::Integer(2)).is_err()) ;
    }

    #[test]
    fn test_cannot_define_variable_twice() {
        let mut env = LexicalEnvironment::new();
        let key = new_symbol("key");

        env.define_variable(&key, Value::Integer(1)).unwrap();
        assert!(env.define_variable(&key, Value::Integer(1)).is_err());
    }

    #[test]
    fn test_cannot_define_function_twice() {
        let mut env = LexicalEnvironment::new();
        let key = new_symbol("key");

        env.define_function(&key, Value::Integer(1)).unwrap();
        assert!(env.define_function(&key, Value::Integer(1)).is_err());
    }

    #[test]
    fn test_able_to_make_child_parent_relationship() {
        let mut env1 = LexicalEnvironment::new();
        let mut env2 = LexicalEnvironment::new();

        let id1 = EnvironmentId::new(1);
        let id2 = EnvironmentId::new(2);

        env1.children.push(id2);
        env2.set_parent(id1);
    }
}
