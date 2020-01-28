use std::collections::HashMap;

use crate::interpreter::value::Value;

#[derive(Debug, Clone, Copy)]
pub struct EnvironmentId {
    index: usize
}

impl EnvironmentId {
    pub fn new(index: usize) -> EnvironmentId {
        EnvironmentId {
            index,
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }
}

// todo: change HashMap<String, Value> to HashMap<Symbol, Value>
pub struct LexicalEnvironment {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Value>,
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

fn lookup_value<'a, 'b>(map: &'a HashMap<String, Value>, name: &'b str) -> Option<&'a Value> {
    for (value_name, value) in map {
        if value_name == name {
            return Some(value);
        }
    }

    None
}

fn set_value(map: &mut HashMap<String, Value>, name: &str, value: Value) {
    match map.get_mut(name) {
        Some(value_ref) => {
            *value_ref = value;
        },
        None => {
            map.insert(name.to_string(), value);
        }
    };
}

fn has_value(map: &HashMap<String, Value>, name: &str) -> bool {
    map.contains_key(name)
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

    pub fn has_variable(&self, name: &str) -> bool {
        has_value(&self.variables, name)
    }

    pub fn has_function(&self, name: &str) -> bool {
        has_value(&self.functions, name)
    }

    pub fn lookup_variable(&self, name: &str) -> Option<&Value> {
        let result = lookup_value(&self.variables, name);

        if let Some(found_value) = result {
            Some(found_value)
        } else {
            None
        }
    }

    pub fn lookup_function(&self, name: &str) -> Option<&Value> {
        let result = lookup_value(&self.functions, name);

        if let Some(found_value) = result {
            Some(found_value)
        } else {
            None
        }
    }

    pub fn define_variable(&mut self, name: &str, value: Value) -> Result<(), ()> {
        if !self.has_variable(name) {
            set_value(&mut self.variables, name, value);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn define_function(&mut self, name: &str, value: Value) -> Result<(), ()> {
        if !self.has_function(name) {
            set_value(&mut self.functions, name, value);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn set_variable(&mut self, name: &str, value: Value) -> Result<(), ()> {
        if self.has_variable(name) {
            set_value(&mut self.variables, name, value);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn set_function(&mut self, name: &str, value: Value) -> Result<(), ()> {
        if self.has_function(name) {
            set_value(&mut self.functions, name, value);
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_makes_new_bindings() {
        let mut env = LexicalEnvironment::new();
        let key = String::from("key");

        assert!(!env.has_variable(&key));
        env.define_variable(&key, Value::Integer(1));
        assert!(env.has_variable(&key));
        assert_eq!(&Value::Integer(1), env.lookup_variable(&key).unwrap());

        assert!(!env.has_function(&key));
        env.define_function(&key, Value::Integer(1));
        assert!(env.has_function(&key));
        assert_eq!(&Value::Integer(1), env.lookup_function(&key).unwrap());
    }

    #[test]
    fn test_makes_updates_bindings() {
        let mut env = LexicalEnvironment::new();
        let key = String::from("key");

        env.define_variable(&key, Value::Integer(1));
        env.define_function(&key, Value::Integer(1));

        env.set_variable(&key, Value::Integer(2));
        env.set_function(&key, Value::Integer(2));

        assert_eq!(&Value::Integer(2), env.lookup_variable(&key).unwrap());
        assert_eq!(&Value::Integer(2), env.lookup_function(&key).unwrap());
    }

    #[test]
    fn test_cannot_set_to_not_defined_variable() {
        let mut env = LexicalEnvironment::new();
        let key = String::from("key");

        assert!(env.set_variable(&key, Value::Integer(2)).is_err());
    }

    #[test]
    fn test_cannot_set_to_not_defined_function() {
        let mut env = LexicalEnvironment::new();
        let key = String::from("key");

       assert!(env.set_function(&key, Value::Integer(2)).is_err()) ;
    }

    #[test]
    fn test_cannot_define_variable_twice() {
        let mut env = LexicalEnvironment::new();
        let key = String::from("key");

        env.define_variable(&key, Value::Integer(1));
        assert!(env.define_variable(&key, Value::Integer(1)).is_err());
    }

    #[test]
    fn test_cannot_define_function_twice() {
        let mut env = LexicalEnvironment::new();
        let key = String::from("key");

        env.define_function(&key, Value::Integer(1));
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
