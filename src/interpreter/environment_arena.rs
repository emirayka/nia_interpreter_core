use crate::interpreter::environment::{
    LexicalEnvironment,
    EnvironmentId
};
use crate::interpreter::value::Value;

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

    pub fn has_variable(&self, id: EnvironmentId, name: &str) -> bool {
        let env = self.get(id).unwrap();

        env.has_variable(name)
    }

    pub fn has_function(&self, id: EnvironmentId, name: &str) -> bool {
        let env = self.get(id).unwrap();

        env.has_function(name)
    }

    pub fn lookup_variable(&self, id: EnvironmentId, name: &str) -> Option<&Value>{
        let env = self.get(id).unwrap();

        match env.lookup_variable(name) {
            Some(result) => Some(result),
            None => {
                match env.get_parent() {
                    Some(parent_id) => self.lookup_variable(parent_id, name),
                    _ => None
                }
            },
        }
    }

    pub fn lookup_function(&self, id: EnvironmentId, name: &str) -> Option<&Value> {
        let env = self.get(id).unwrap();

        match env.lookup_function(name) {
            Some(result) => Some(result),
            None => {
                match env.get_parent() {
                    Some(parent_id) => self.lookup_function(parent_id, name),
                    _ => None
                }
            },
        }
    }

    pub fn define_variable(&mut self, id: EnvironmentId, name: &str, value: Value) -> Result<(), ()> {
        let env = self.get_mut(id).unwrap();

        env.define_variable(name, value)
    }

    pub fn define_function(&mut self, id: EnvironmentId, name: &str, value: Value) -> Result<(), ()> {
        let env = self.get_mut(id).unwrap();

        env.define_function(name, value)
    }

    pub fn set_variable(&mut self, id: EnvironmentId, name: &str, value: Value) -> Result<(), ()> {
        let env = self.get_mut(id).unwrap();

        if env.has_variable(name) {
            env.set_variable(name, value);
            Ok(())
        } else if let Some(parent_id) = env.get_parent() {
            self.set_variable(parent_id, name, value)
        } else {
            Err(())
        }
    }

    pub fn set_function(&mut self, id: EnvironmentId, name: &str, value: Value) -> Result<(), ()> {
        let env = self.get_mut(id).unwrap();

        if env.has_function(name) {
            env.set_function(name, value);
            Ok(())
        } else if let Some(parent_id) = env.get_parent() {
            self.set_function(parent_id, name, value)
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_works_correctly_for_a_single_environment() {
        let mut arena = Arena::new();
        let key = "test";

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
        let key = "test";

        let parent_id = arena.alloc();

        arena.define_variable(parent_id, key, Value::Integer(1));
        assert_eq!(&Value::Integer(1), arena.lookup_variable(parent_id, key).unwrap());
        arena.set_variable(parent_id, key, Value::Integer(2));
        assert_eq!(&Value::Integer(2), arena.lookup_variable(parent_id, key).unwrap());

        arena.define_function(parent_id, key, Value::Integer(1));
        assert_eq!(&Value::Integer(1), arena.lookup_function(parent_id, key).unwrap());
        arena.set_function(parent_id, key, Value::Integer(2));
        assert_eq!(&Value::Integer(2), arena.lookup_function(parent_id, key).unwrap());
    }

    #[test]
    fn test_cannot_set_to_not_defined_variable() {
        let mut arena = Arena::new();

        let env_id = arena.alloc();
        let key = String::from("key");

        assert!(arena.set_variable(env_id, &key, Value::Integer(2)).is_err());
    }

    #[test]
    fn test_cannot_set_to_not_defined_function() {
        let mut arena = Arena::new();

        let env_id = arena.alloc();
        let key = String::from("key");

        assert!(arena.set_function(env_id, &key, Value::Integer(2)).is_err());
    }

    #[test]
    fn test_cannot_define_variable_twice() {
        let mut arena = Arena::new();

        let env_id = arena.alloc();
        let key = String::from("key");

        arena.define_variable(env_id, &key, Value::Integer(1));
        assert!(arena.define_variable(env_id, &key, Value::Integer(1)).is_err());
    }

    #[test]
    fn test_cannot_define_function_twice() {
        let mut arena = Arena::new();

        let env_id = arena.alloc();
        let key = String::from("key");

        arena.define_function(env_id, &key, Value::Integer(1));
        assert!(arena.define_function(env_id, &key, Value::Integer(1)).is_err());
    }

    #[test]
    fn test_lookups_from_parents_works_correctly() {
        let mut arena = Arena::new();

        let parent_id = arena.alloc();
        let child_id = arena.alloc_child(parent_id);

        // variable
        arena.define_variable(parent_id, "parent_test", Value::String("parent".to_string()));
        arena.define_variable(child_id, "child_test", Value::String("child".to_string()));

        assert_eq!(&Value::String("parent".to_string()), arena.lookup_variable(parent_id, "parent_test").unwrap());
        assert_eq!(&Value::String("parent".to_string()), arena.lookup_variable(child_id, "parent_test").unwrap());
        assert_eq!(None, arena.lookup_variable(parent_id, "child_test"));
        assert_eq!(&Value::String("child".to_string()), arena.lookup_variable(child_id, "child_test").unwrap());

        // function
        arena.define_function(parent_id, "parent_test", Value::String("parent".to_string()));
        arena.define_function(child_id, "child_test", Value::String("child".to_string()));

        assert_eq!(&Value::String("parent".to_string()), arena.lookup_function(parent_id, "parent_test").unwrap());
        assert_eq!(&Value::String("parent".to_string()), arena.lookup_function(child_id, "parent_test").unwrap());
        assert_eq!(None, arena.lookup_function(parent_id, "child_test"));
        assert_eq!(&Value::String("child".to_string()), arena.lookup_function(child_id, "child_test").unwrap());
    }

    #[test]
    fn test_when_defined_only_in_parent_set_only_in_parent() {
        let mut arena = Arena::new();
        let key = "test";

        let parent_id = arena.alloc();
        let child_id = arena.alloc_child(parent_id);

        arena.define_variable(parent_id, "test", Value::String("parent".to_string()));
        arena.set_variable(child_id, "test", Value::Integer(1));
        assert_eq!(&Value::Integer(1), arena.lookup_variable(parent_id,"test").unwrap());

        arena.define_function(parent_id, "test", Value::String("parent".to_string()));
        arena.set_function(child_id, "test", Value::Integer(1));
        assert_eq!(&Value::Integer(1), arena.lookup_function(child_id,"test").unwrap());
    }

    // todo: add check of variable/function names?
}
