use crate::interpreter::environment::{
    LexicalEnvironment,
    EnvironmentId
};
use crate::interpreter::value::Value;
use crate::interpreter::symbol::Symbol;
use crate::interpreter::error::Error;

pub struct Arena {
    contestants: Vec<LexicalEnvironment>,
}

impl Arena {
    fn get(&self, id: EnvironmentId) -> Option<&LexicalEnvironment> {
        self.contestants.get(id.get_index())
    }

    fn get_mut(&mut self, id: EnvironmentId) -> Option<&mut LexicalEnvironment> {
        self.contestants.get_mut(id.get_index())
    }
}

impl Arena {
    pub fn new() -> Arena {
        Arena {
            contestants: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> EnvironmentId {
        let env = LexicalEnvironment::new();
        self.contestants.push(env);

        let index = self.contestants.len() - 1;
        let id = EnvironmentId::new(index);

        id
    }

    pub fn alloc_child(&mut self, parent_id: EnvironmentId) -> EnvironmentId {
        let child_id = self.alloc();

        if let Some(parent) = self.get_mut(parent_id) {
            parent.add_child(child_id);
        } else {
            unreachable!();
        }

        if let Some(child) = self.get_mut(child_id) {
            child.set_parent(parent_id);
        } else {
            unreachable!();
        }

        child_id
    }

    pub fn has_variable(&self, id: EnvironmentId, symbol: &Symbol) -> bool {
        let env = self.get(id).unwrap();

        env.has_variable(symbol)
    }

    pub fn has_function(&self, id: EnvironmentId, symbol: &Symbol) -> bool {
        let env = self.get(id).unwrap();

        env.has_function(symbol)
    }

    pub fn lookup_variable(&self, id: EnvironmentId, symbol: &Symbol) -> Option<&Value>{
        let env = self.get(id).unwrap();

        match env.lookup_variable(symbol) {
            Some(result) => Some(result),
            None => {
                match env.get_parent() {
                    Some(parent_id) => self.lookup_variable(parent_id, symbol),
                    _ => None
                }
            },
        }
    }

    pub fn lookup_function(&self, id: EnvironmentId, symbol: &Symbol) -> Option<&Value> {
        let env = self.get(id).unwrap();

        match env.lookup_function(symbol) {
            Some(result) => Some(result),
            None => {
                match env.get_parent() {
                    Some(parent_id) => self.lookup_function(parent_id, symbol),
                    _ => None
                }
            },
        }
    }

    pub fn define_variable(&mut self, id: EnvironmentId, symbol: &Symbol, value: Value) -> Result<(), Error> {
        let env = self.get_mut(id).unwrap();

        env.define_variable(symbol, value)
    }

    pub fn define_function(&mut self, id: EnvironmentId, symbol: &Symbol, value: Value) -> Result<(), Error> {
        let env = self.get_mut(id).unwrap();

        env.define_function(symbol, value)
    }

    pub fn set_variable(&mut self, id: EnvironmentId, symbol: &Symbol, value: Value) -> Result<(), Error> {
        let env = self.get_mut(id).unwrap();

        if env.has_variable(symbol) {
            env.set_variable(symbol, value);
            Ok(())
        } else if let Some(parent_id) = env.get_parent() {
            self.set_variable(parent_id, symbol, value)
        } else {
            Err(Error::empty())
        }
    }

    pub fn set_function(&mut self, id: EnvironmentId, symbol: &Symbol, value: Value) -> Result<(), Error> {
        let env = self.get_mut(id).unwrap();

        if env.has_function(symbol) {
            env.set_function(symbol, value);
            Ok(())
        } else if let Some(parent_id) = env.get_parent() {
            self.set_function(parent_id, symbol, value)
        } else {
            Err(Error::empty())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_works_correctly_for_a_single_environment() {
        let mut arena = Arena::new();
        let key = &Symbol::from("test");

        let parent_id = arena.alloc();

        assert!(!arena.has_variable(parent_id, key));
        arena.define_variable(parent_id, key, Value::Integer(1));
        assert!(arena.has_variable(parent_id, key));
        assert_eq!(&Value::Integer(1), arena.lookup_variable(parent_id, key).unwrap());

        assert!(!arena.has_function(parent_id, key));
        arena.define_function(parent_id, key, Value::Integer(1));
        assert!(arena.has_function(parent_id, key));
        assert_eq!(&Value::Integer(1), arena.lookup_function(parent_id, key).unwrap());
    }

    #[test]
    fn test_set_works_correctly_for_a_single_environment() {
        let mut arena = Arena::new();
        let key = Symbol::from("test");

        let parent_id = arena.alloc();

        arena.define_variable(parent_id, &key, Value::Integer(1));
        assert_eq!(&Value::Integer(1), arena.lookup_variable(parent_id, &key).unwrap());
        arena.set_variable(parent_id, &key, Value::Integer(2));
        assert_eq!(&Value::Integer(2), arena.lookup_variable(parent_id, &key).unwrap());

        arena.define_function(parent_id, &key, Value::Integer(1));
        assert_eq!(&Value::Integer(1), arena.lookup_function(parent_id, &key).unwrap());
        arena.set_function(parent_id, &key, Value::Integer(2));
        assert_eq!(&Value::Integer(2), arena.lookup_function(parent_id, &key).unwrap());
    }

    #[test]
    fn test_cannot_set_to_not_defined_variable() {
        let mut arena = Arena::new();

        let env_id = arena.alloc();
        let key = Symbol::from("key");

        assert!(arena.set_variable(env_id, &key, Value::Integer(2)).is_err());
    }

    #[test]
    fn test_cannot_set_to_not_defined_function() {
        let mut arena = Arena::new();

        let env_id = arena.alloc();
        let key = Symbol::from("key");

        assert!(arena.set_function(env_id, &key, Value::Integer(2)).is_err());
    }

    #[test]
    fn test_cannot_define_variable_twice() {
        let mut arena = Arena::new();

        let env_id = arena.alloc();
        let key = Symbol::from("key");

        arena.define_variable(env_id, &key, Value::Integer(1));
        assert!(arena.define_variable(env_id, &key, Value::Integer(1)).is_err());
    }

    #[test]
    fn test_cannot_define_function_twice() {
        let mut arena = Arena::new();

        let env_id = arena.alloc();
        let key = Symbol::from("key");

        arena.define_function(env_id, &key, Value::Integer(1));
        assert!(arena.define_function(env_id, &key, Value::Integer(1)).is_err());
    }

    #[test]
    fn test_lookups_from_parents_works_correctly() {
        let mut arena = Arena::new();

        let parent_id = arena.alloc();
        let child_id = arena.alloc_child(parent_id);

        let parent_key = Symbol::from("parent_test");
        let child_key = Symbol::from("child_test");

        // variable
        arena.define_variable(parent_id, &parent_key, Value::String("parent".to_string()));
        arena.define_variable(child_id, &child_key, Value::String("child".to_string()));

        assert_eq!(&Value::String("parent".to_string()), arena.lookup_variable(parent_id, &parent_key).unwrap());
        assert_eq!(&Value::String("parent".to_string()), arena.lookup_variable(child_id, &parent_key).unwrap());
        assert_eq!(None, arena.lookup_variable(parent_id, &child_key));
        assert_eq!(&Value::String("child".to_string()), arena.lookup_variable(child_id, &child_key).unwrap());

        // function
        arena.define_function(parent_id, &parent_key, Value::String("parent".to_string()));
        arena.define_function(child_id, &child_key, Value::String("child".to_string()));

        assert_eq!(&Value::String("parent".to_string()), arena.lookup_function(parent_id, &parent_key).unwrap());
        assert_eq!(&Value::String("parent".to_string()), arena.lookup_function(child_id, &parent_key).unwrap());
        assert_eq!(None, arena.lookup_function(parent_id, &child_key));
        assert_eq!(&Value::String("child".to_string()), arena.lookup_function(child_id, &child_key).unwrap());
    }

    #[test]
    fn test_when_defined_only_in_parent_set_only_in_parent() {
        let mut arena = Arena::new();
        let key = Symbol::from("test");

        let parent_id = arena.alloc();
        let child_id = arena.alloc_child(parent_id);

        arena.define_variable(parent_id, &key, Value::String("parent".to_string()));
        arena.set_variable(child_id, &key, Value::Integer(1));
        assert_eq!(&Value::Integer(1), arena.lookup_variable(parent_id,&key).unwrap());

        arena.define_function(parent_id, &key, Value::String("parent".to_string()));
        arena.set_function(child_id, &key, Value::Integer(1));
        assert_eq!(&Value::Integer(1), arena.lookup_function(child_id,&key).unwrap());
    }

    // todo: add check of variable/function names?
    // wat is it?
}
