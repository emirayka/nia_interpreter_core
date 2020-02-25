use std::collections::HashMap;

use crate::interpreter::value::Value;
use crate::interpreter::symbol::SymbolId;
use crate::interpreter::environment::environment::LexicalEnvironment;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnvironmentId {
    index: usize,
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
    arena: HashMap<EnvironmentId, LexicalEnvironment>,
    next_id: usize,
}

impl EnvironmentArena {
    fn get(&self, id: EnvironmentId) -> Result<&LexicalEnvironment, ()> {
        self.arena.get(&id).ok_or(())
    }

    fn get_mut(&mut self, id: EnvironmentId) -> Result<&mut LexicalEnvironment, ()> {
        self.arena.get_mut(&id).ok_or(())
    }
}

impl EnvironmentArena {
    pub fn new() -> EnvironmentArena {
        EnvironmentArena {
            arena: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn alloc(&mut self) -> EnvironmentId {
        let env = LexicalEnvironment::new();

        let id = EnvironmentId::new(self.next_id);

        self.arena.insert(id, env);
        self.next_id += 1;

        id
    }

    pub fn alloc_child(&mut self, parent_id: EnvironmentId) -> Result<EnvironmentId, ()> {
        let child_id = self.alloc();

        if let Ok(parent) = self.get_mut(parent_id) {
            parent.add_child(child_id);
        } else {
            return Err(())
        };

        if let Ok(child) = self.get_mut(child_id) {
            child.set_parent(parent_id);
        } else {
            return Err(())
        }

        Ok(child_id)
    }

    pub fn has_variable(&self, id: EnvironmentId, symbol_id: SymbolId) -> Result<bool, ()> {
        let env = self.get(id)?;

        Ok(env.has_variable(symbol_id))
    }

    pub fn has_function(&self, id: EnvironmentId, symbol_id: SymbolId) -> Result<bool, ()> {
        let env = self.get(id)?;

        Ok(env.has_function(symbol_id))
    }

    pub fn lookup_variable(&self, id: EnvironmentId, symbol_id: SymbolId) -> Result<Option<Value>, ()> {
        let env = self.get(id)?;

        match env.lookup_variable(symbol_id) {
            Some(result) => Ok(Some(result)),
            None => {
                match env.get_parent() {
                    Some(parent_id) => self.lookup_variable(parent_id, symbol_id),
                    _ => Ok(None)
                }
            },
        }
    }

    pub fn lookup_function(&self, id: EnvironmentId, symbol_id: SymbolId) -> Result<Option<Value>, ()> {
        let env = self.get(id)?;

        match env.lookup_function(symbol_id) {
            Some(result) => Ok(Some(result)),
            None => {
                match env.get_parent() {
                    Some(parent_id) => self.lookup_function(parent_id, symbol_id),
                    _ => Ok(None)
                }
            },
        }
    }

    pub fn define_variable(
        &mut self,
        id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), ()> {
        let env = self.get_mut(id)?;

        env.define_variable(symbol_id, value)
    }

    pub fn define_function(
        &mut self,
        id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), ()> {
        let env = self.get_mut(id)?;

        env.define_function(symbol_id, value)
    }

    pub fn set_environment_variable(
        &mut self,
        id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), ()> {
        let env = self.get_mut(id)?;

        if env.has_variable(symbol_id) {
            env.set_variable(symbol_id, value)
        } else {
            Err(())
        }
    }

    pub fn set_environment_function(
        &mut self,
        id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), ()> {
        let env = self.get_mut(id)?;

        if env.has_function(symbol_id) {
            env.set_function(symbol_id, value)
        } else {
            Err(())
        }
    }

    pub fn set_variable(
        &mut self,
        id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), ()> {
        let env = self.get_mut(id)?;

        if env.has_variable(symbol_id) {
            env.set_variable(symbol_id, value)
        } else if let Some(parent_id) = env.get_parent() {
            self.set_variable(parent_id, symbol_id, value)
        } else {
            Err(())
        }
    }

    pub fn set_function(
        &mut self,
        id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), ()> {
        let env = self.get_mut(id)?;

        if env.has_function(symbol_id) {
            env.set_function(symbol_id, value)
        } else if let Some(parent_id) = env.get_parent() {
            self.set_function(parent_id, symbol_id, value)
        } else {
            Err(())
        }
    }

    pub fn lookup_environment_by_variable(
        &self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId
    ) -> Result<Option<EnvironmentId>, ()> {
        let env = self.get(environment_id)?;

        match env.has_variable(variable_symbol_id) {
            true => Ok(Some(environment_id)),
            false => match self.get(environment_id)?.get_parent() {
                Some(parent) => self.lookup_environment_by_variable(parent, variable_symbol_id),
                None => Ok(None)
            }
        }
    }

    pub fn lookup_environment_by_function(
        &self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId
    ) -> Result<Option<EnvironmentId>, ()> {
        let env = self.get(environment_id)?;

        match env.has_function(function_symbol_id) {
            true => Ok(Some(environment_id)),
            false => match env.get_parent() {
                Some(parent) => self.lookup_environment_by_function(parent, function_symbol_id),
                None => Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_works_correctly_for_a_single_environment() {
        let mut arena = EnvironmentArena::new();
        let key = SymbolId::new(0);

        let parent_id = arena.alloc();

        assert!(!arena.has_variable(parent_id, key).unwrap());
        arena.define_variable(parent_id, key, Value::Integer(1)).unwrap();
        assert!(arena.has_variable(parent_id, key).unwrap());
        assert_eq!(Ok(Some(Value::Integer(1))), arena.lookup_variable(parent_id, key));

        assert!(!arena.has_function(parent_id, key).unwrap());
        arena.define_function(parent_id, key, Value::Integer(1)).unwrap();
        assert!(arena.has_function(parent_id, key).unwrap());
        assert_eq!(Ok(Some(Value::Integer(1))), arena.lookup_function(parent_id, key));
    }

    #[test]
    fn test_set_works_correctly_for_a_single_environment() {
        let mut arena = EnvironmentArena::new();
        let key = SymbolId::new(0);

        let parent_id = arena.alloc();

        arena.define_variable(parent_id, key, Value::Integer(1)).unwrap();
        assert_eq!(Ok(Some(Value::Integer(1))), arena.lookup_variable(parent_id, key));
        arena.set_variable(parent_id, key, Value::Integer(2)).unwrap();
        assert_eq!(Ok(Some(Value::Integer(2))), arena.lookup_variable(parent_id, key));

        arena.define_function(parent_id, key, Value::Integer(1)).unwrap();
        assert_eq!(Ok(Some(Value::Integer(1))), arena.lookup_function(parent_id, key));
        arena.set_function(parent_id, key, Value::Integer(2)).unwrap();
        assert_eq!(Ok(Some(Value::Integer(2))), arena.lookup_function(parent_id, key));
    }

    #[test]
    fn test_cannot_set_to_not_defined_variable() {
        let mut arena = EnvironmentArena::new();

        let env_id = arena.alloc();
        let key = SymbolId::new(0);

        assert!(arena.set_variable(env_id, key, Value::Integer(2)).is_err());
    }

    #[test]
    fn test_cannot_set_to_not_defined_function() {
        let mut arena = EnvironmentArena::new();

        let env_id = arena.alloc();
        let key = SymbolId::new(0);

        assert!(arena.set_function(env_id, key, Value::Integer(2)).is_err());
    }

    #[test]
    fn test_cannot_define_variable_twice() {
        let mut arena = EnvironmentArena::new();

        let env_id = arena.alloc();
        let key = SymbolId::new(0);

        arena.define_variable(env_id, key, Value::Integer(1)).unwrap();
        assert!(arena.define_variable(env_id, key, Value::Integer(1)).is_err());
    }

    #[test]
    fn test_cannot_define_function_twice() {
        let mut arena = EnvironmentArena::new();

        let env_id = arena.alloc();
        let key = SymbolId::new(0);

        arena.define_function(env_id, key, Value::Integer(1)).unwrap();
        assert!(arena.define_function(env_id, key, Value::Integer(1)).is_err());
    }

    #[test]
    fn test_lookups_from_parents_works_correctly() {
        let mut arena = EnvironmentArena::new();

        let parent_id = arena.alloc();
        let child_id = arena.alloc_child(parent_id).unwrap();

        let parent_key = SymbolId::new(0);
        let child_key = SymbolId::new(1);

        let parent_value = Value::Integer(1);
        let child_value = Value::Integer(2);

        // variable
        arena.define_variable(parent_id, parent_key, parent_value).unwrap();
        arena.define_variable(child_id, child_key, child_value).unwrap();

        assert_eq!(Ok(Some(parent_value)), arena.lookup_variable(parent_id, parent_key));
        assert_eq!(Ok(Some(parent_value)), arena.lookup_variable(child_id, parent_key));
        assert_eq!(Ok(None), arena.lookup_variable(parent_id, child_key));
        assert_eq!(Ok(Some(child_value)), arena.lookup_variable(child_id, child_key));

        // function
        arena.define_function(parent_id, parent_key, parent_value).unwrap();
        arena.define_function(child_id, child_key, child_value).unwrap();

        assert_eq!(Ok(Some(parent_value)), arena.lookup_function(parent_id, parent_key));
        assert_eq!(Ok(Some(parent_value)), arena.lookup_function(child_id, parent_key));
        assert_eq!(Ok(None), arena.lookup_function(parent_id, child_key));
        assert_eq!(Ok(Some(child_value)), arena.lookup_function(child_id, child_key));
    }

    #[test]
    fn test_when_defined_only_in_parent_set_only_in_parent() {
        let mut arena = EnvironmentArena::new();
        let key = SymbolId::new(0);

        let parent_id = arena.alloc();
        let child_id = arena.alloc_child(parent_id).unwrap();

        let parent_value = Value::Integer(1);
        let child_value = Value::Integer(2);

        arena.define_variable(parent_id, key, parent_value).unwrap();
        arena.set_variable(child_id, key, child_value).unwrap();
        assert_eq!(Ok(Some(child_value)), arena.lookup_variable(parent_id, key));

        arena.define_function(parent_id, key, parent_value).unwrap();
        arena.set_function(child_id, key, child_value).unwrap();
        assert_eq!(Ok(Some(child_value)), arena.lookup_function(child_id,key));
    }

    #[cfg(test)]
    mod lookup_environment_by_variable {
        use super::*;

        #[test]
        fn test_returns_current_environment_when_variable_is_defined_here() {
            let mut arena = EnvironmentArena::new();

            let parent_id = arena.alloc();
            let child_id = arena.alloc_child(parent_id).unwrap();

            let variable_name = SymbolId::new(0);

            arena.define_variable(child_id, variable_name, Value::Integer(1)).unwrap();

            assert_eq!(
                Ok(Some(child_id)),
                arena.lookup_environment_by_variable(
                    child_id,
                    variable_name
                )
            );
        }

        #[test]
        fn test_returns_parent_environment_when_variable_is_defined_here() {
            let mut arena = EnvironmentArena::new();

            let parent_id = arena.alloc();
            let child_id = arena.alloc_child(parent_id).unwrap();

            let variable_name = SymbolId::new(0);

            arena.define_variable(parent_id, variable_name, Value::Integer(1)).unwrap();

            assert_eq!(
                Ok(Some(parent_id)),
                arena.lookup_environment_by_variable(
                    child_id,
                    variable_name
                )
            );
        }

        #[test]
        fn test_returns_parent_environment_when_variable_is_defined_2() {
            let mut arena = EnvironmentArena::new();

            let parent_id = arena.alloc();
            let child_id = arena.alloc_child(parent_id).unwrap();
            let child_child_id = arena.alloc_child(child_id).unwrap();

            let variable_name = SymbolId::new(0);

            arena.define_variable(parent_id, variable_name, Value::Integer(1)).unwrap();

            assert_eq!(
                Ok(Some(parent_id)),
                arena.lookup_environment_by_variable(
                    child_child_id,
                    variable_name
                )
            );
        }

        #[test]
        fn test_returns_none_when_variable_is_defined_nowhere() {
            let mut arena = EnvironmentArena::new();

            let parent_id = arena.alloc();
            let child_id = arena.alloc_child(parent_id).unwrap();
            let child_child_id = arena.alloc_child(child_id).unwrap();

            let variable_name = SymbolId::new(0);

            assert_eq!(
                Ok(None),
                arena.lookup_environment_by_variable(
                    child_child_id,
                    variable_name
                )
            );
        }
    }
}
