use crate::interpreter::value::Value;
use crate::interpreter::symbol::Symbol;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment::LexicalEnvironment;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub struct EnvironmentArena {
    contestants: Vec<LexicalEnvironment>,
}

impl EnvironmentArena {
    fn get(&self, id: EnvironmentId) -> Option<&LexicalEnvironment> {
        self.contestants.get(id.get_index())
    }

    fn get_mut(&mut self, id: EnvironmentId) -> Option<&mut LexicalEnvironment> {
        self.contestants.get_mut(id.get_index())
    }
}

impl EnvironmentArena {
    pub fn new() -> EnvironmentArena {
        EnvironmentArena {
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
            match env.set_variable(symbol, value) {
                Ok(()) => Ok(()),
                Err(error) => Err(error)
            }
        } else if let Some(parent_id) = env.get_parent() {
            self.set_variable(parent_id, symbol, value)
        } else {
            Err(Error::empty())
        }
    }

    pub fn set_function(&mut self, id: EnvironmentId, symbol: &Symbol, value: Value) -> Result<(), Error> {
        let env = self.get_mut(id).unwrap();

        if env.has_function(symbol) {
            match env.set_function(symbol, value) {
                Ok(()) => Ok(()),
                Err(error) => Err(error)
            }
        } else if let Some(parent_id) = env.get_parent() {
            self.set_function(parent_id, symbol, value)
        } else {
            Err(Error::empty())
        }
    }

    pub fn lookup_environment_by_variable(
        &self,
        environment: EnvironmentId,
        variable_name: &Symbol
    ) -> Option<EnvironmentId > {
        match self.has_variable(environment, variable_name) {
            true => Some(environment),
            false => match self.get(environment).unwrap().get_parent() {
                Some(parent) => self.lookup_environment_by_variable(parent, variable_name),
                None => None
            }
        }
    }

    pub fn lookup_environment_by_function(
        &self,
        environment: EnvironmentId,
        function_name: &Symbol
    ) -> Option<EnvironmentId > {
        match self.has_function(environment, function_name) {
            true => Some(environment),
            false => match self.get(environment).unwrap().get_parent() {
                Some(parent) => self.lookup_environment_by_function(parent, function_name),
                None => None
            }
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
    fn test_define_works_correctly_for_a_single_environment() {
        let mut arena = EnvironmentArena::new();
        let key = new_symbol("test");

        let parent_id = arena.alloc();

        assert!(!arena.has_variable(parent_id, &key));
        arena.define_variable(parent_id, &key, Value::Integer(1)).unwrap();
        assert!(arena.has_variable(parent_id, &key));
        assert_eq!(&Value::Integer(1), arena.lookup_variable(parent_id, &key).unwrap());

        assert!(!arena.has_function(parent_id, &key));
        arena.define_function(parent_id, &key, Value::Integer(1)).unwrap();
        assert!(arena.has_function(parent_id, &key));
        assert_eq!(&Value::Integer(1), arena.lookup_function(parent_id, &key).unwrap());
    }

    #[test]
    fn test_set_works_correctly_for_a_single_environment() {
        let mut arena = EnvironmentArena::new();
        let key = new_symbol("test");

        let parent_id = arena.alloc();

        arena.define_variable(parent_id, &key, Value::Integer(1)).unwrap();
        assert_eq!(&Value::Integer(1), arena.lookup_variable(parent_id, &key).unwrap());
        arena.set_variable(parent_id, &key, Value::Integer(2)).unwrap();
        assert_eq!(&Value::Integer(2), arena.lookup_variable(parent_id, &key).unwrap());

        arena.define_function(parent_id, &key, Value::Integer(1)).unwrap();
        assert_eq!(&Value::Integer(1), arena.lookup_function(parent_id, &key).unwrap());
        arena.set_function(parent_id, &key, Value::Integer(2)).unwrap();
        assert_eq!(&Value::Integer(2), arena.lookup_function(parent_id, &key).unwrap());
    }

    #[test]
    fn test_cannot_set_to_not_defined_variable() {
        let mut arena = EnvironmentArena::new();

        let env_id = arena.alloc();
        let key = new_symbol("key");

        assert!(arena.set_variable(env_id, &key, Value::Integer(2)).is_err());
    }

    #[test]
    fn test_cannot_set_to_not_defined_function() {
        let mut arena = EnvironmentArena::new();

        let env_id = arena.alloc();
        let key = new_symbol("key");

        assert!(arena.set_function(env_id, &key, Value::Integer(2)).is_err());
    }

    #[test]
    fn test_cannot_define_variable_twice() {
        let mut arena = EnvironmentArena::new();

        let env_id = arena.alloc();
        let key = new_symbol("key");

        arena.define_variable(env_id, &key, Value::Integer(1)).unwrap();
        assert!(arena.define_variable(env_id, &key, Value::Integer(1)).is_err());
    }

    #[test]
    fn test_cannot_define_function_twice() {
        let mut arena = EnvironmentArena::new();

        let env_id = arena.alloc();
        let key = new_symbol("key");

        arena.define_function(env_id, &key, Value::Integer(1)).unwrap();
        assert!(arena.define_function(env_id, &key, Value::Integer(1)).is_err());
    }

    #[test]
    fn test_lookups_from_parents_works_correctly() {
        let mut arena = EnvironmentArena::new();

        let parent_id = arena.alloc();
        let child_id = arena.alloc_child(parent_id);

        let parent_key = new_symbol("parent_test");
        let child_key = new_symbol("child_test");

        // variable
        arena.define_variable(parent_id, &parent_key, Value::String("parent".to_string())).unwrap();
        arena.define_variable(child_id, &child_key, Value::String("child".to_string())).unwrap();

        assert_eq!(&Value::String("parent".to_string()), arena.lookup_variable(parent_id, &parent_key).unwrap());
        assert_eq!(&Value::String("parent".to_string()), arena.lookup_variable(child_id, &parent_key).unwrap());
        assert_eq!(None, arena.lookup_variable(parent_id, &child_key));
        assert_eq!(&Value::String("child".to_string()), arena.lookup_variable(child_id, &child_key).unwrap());

        // function
        arena.define_function(parent_id, &parent_key, Value::String("parent".to_string())).unwrap();
        arena.define_function(child_id, &child_key, Value::String("child".to_string())).unwrap();

        assert_eq!(&Value::String("parent".to_string()), arena.lookup_function(parent_id, &parent_key).unwrap());
        assert_eq!(&Value::String("parent".to_string()), arena.lookup_function(child_id, &parent_key).unwrap());
        assert_eq!(None, arena.lookup_function(parent_id, &child_key));
        assert_eq!(&Value::String("child".to_string()), arena.lookup_function(child_id, &child_key).unwrap());
    }

    #[test]
    fn test_when_defined_only_in_parent_set_only_in_parent() {
        let mut arena = EnvironmentArena::new();
        let key = new_symbol("test");

        let parent_id = arena.alloc();
        let child_id = arena.alloc_child(parent_id);

        arena.define_variable(parent_id, &key, Value::String("parent".to_string())).unwrap();
        arena.set_variable(child_id, &key, Value::Integer(1)).unwrap();
        assert_eq!(&Value::Integer(1), arena.lookup_variable(parent_id,&key).unwrap());

        arena.define_function(parent_id, &key, Value::String("parent".to_string())).unwrap();
        arena.set_function(child_id, &key, Value::Integer(1)).unwrap();
        assert_eq!(&Value::Integer(1), arena.lookup_function(child_id,&key).unwrap());
    }

    #[cfg(test)]
    mod lookup_environment_by_variable {
        use super::*;

        #[test]
        fn test_returns_current_environment_when_variable_is_defined_here() {
            let mut arena = EnvironmentArena::new();

            let parent_id = arena.alloc();
            let child_id = arena.alloc_child(parent_id);

            let variable_name = new_symbol("test");

            arena.define_variable(child_id, &variable_name, Value::Integer(1)).unwrap();

            assert_eq!(
                Some(child_id),
                arena.lookup_environment_by_variable(
                    child_id,
                    &variable_name)
            );
        }

        #[test]
        fn test_returns_parent_environment_when_variable_is_defined_here() {
            let mut arena = EnvironmentArena::new();

            let parent_id = arena.alloc();
            let child_id = arena.alloc_child(parent_id);

            let variable_name = new_symbol("test");

            arena.define_variable(parent_id, &variable_name, Value::Integer(1)).unwrap();

            assert_eq!(
                Some(parent_id),
                arena.lookup_environment_by_variable(
                    child_id,
                    &variable_name)
            );
        }

        #[test]
        fn test_returns_parent_environment_when_variable_is_defined_2() {
            let mut arena = EnvironmentArena::new();

            let parent_id = arena.alloc();
            let child_id = arena.alloc_child(parent_id);
            let child_child_id = arena.alloc_child(child_id);

            let variable_name = new_symbol("test");

            arena.define_variable(parent_id, &variable_name, Value::Integer(1)).unwrap();

            assert_eq!(
                Some(parent_id),
                arena.lookup_environment_by_variable(
                    child_child_id,
                    &variable_name)
            );
        }

        #[test]
        fn test_returns_none_when_variable_is_defined_nowhere() {
            let mut arena = EnvironmentArena::new();

            let parent_id = arena.alloc();
            let child_id = arena.alloc_child(parent_id);
            let child_child_id = arena.alloc_child(child_id);

            let variable_name = new_symbol("test");

            assert_eq!(
                None,
                arena.lookup_environment_by_variable(
                    child_child_id,
                    &variable_name)
            );
        }
    }

    // todo: add check of variable/function names?
    // wat is it?
}
