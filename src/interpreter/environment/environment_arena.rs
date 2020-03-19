use std::collections::HashMap;

use crate::interpreter::value::Value;
use crate::interpreter::symbol::SymbolId;
use crate::interpreter::environment::environment::LexicalEnvironment;
use crate::interpreter::error::Error;

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

    pub fn get_id(&self) -> usize {
        self.index
    }
}

pub struct EnvironmentArena {
    arena: HashMap<EnvironmentId, LexicalEnvironment>,
    next_id: usize,
}

impl EnvironmentArena {
    fn get(&self, id: EnvironmentId) -> Result<&LexicalEnvironment, Error> {
        self.arena
            .get(&id)
            .ok_or(Error::failure(
                format!("Cannot find an environment with id: {}", id.get_id())
            ))
    }

    fn get_mut(&mut self, id: EnvironmentId) -> Result<&mut LexicalEnvironment, Error> {
        self.arena
            .get_mut(&id)
            .ok_or(Error::failure(
                format!("Cannot find an environment with id: {}", id.get_id())
            ))
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

    pub fn alloc_child(&mut self, parent_id: EnvironmentId) -> Result<EnvironmentId, Error> {
        let child_id = self.alloc();

        {
            let parent = self.get_mut(parent_id)?;
            parent.add_child(child_id);
        }

        {
            let child = self.get_mut(child_id)?;
            child.set_parent(parent_id);
        }

        Ok(child_id)
    }

    pub fn free_environment(&mut self, environment_id: EnvironmentId) -> Result<(), Error> {
        match self.arena.remove(&environment_id) {
            Some(_) => Ok(()),
            _ => Error::failure(
                format!("Cannot find an environment with id: {}", environment_id.get_id())
            ).into_result()
        }
    }

    pub fn has_variable(&self, id: EnvironmentId, symbol_id: SymbolId) -> Result<bool, Error> {
        let env = self.get(id)?;

        Ok(env.has_variable(symbol_id))
    }

    pub fn has_function(&self, id: EnvironmentId, symbol_id: SymbolId) -> Result<bool, Error> {
        let env = self.get(id)?;

        Ok(env.has_function(symbol_id))
    }

    pub fn lookup_variable(&self, id: EnvironmentId, symbol_id: SymbolId) -> Result<Option<Value>, Error> {
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

    pub fn lookup_function(&self, id: EnvironmentId, symbol_id: SymbolId) -> Result<Option<Value>, Error> {
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
    ) -> Result<(), Error> {
        let env = self.get_mut(id)?;

        env.define_variable(symbol_id, value)
    }

    pub fn define_function(
        &mut self,
        id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), Error> {
        let env = self.get_mut(id)?;

        env.define_function(symbol_id, value)
    }

    pub fn set_environment_variable(
        &mut self,
        id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), Error> {
        let env = self.get_mut(id)?;

        env.set_variable(symbol_id, value)
    }

    pub fn set_environment_function(
        &mut self,
        id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), Error> {
        let env = self.get_mut(id)?;

        env.set_function(symbol_id, value)
    }

    pub fn set_variable(
        &mut self,
        id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), Error> {
        let env = self.get_mut(id)?;

        if env.has_variable(symbol_id) {
            env.set_variable(symbol_id, value)
        } else if let Some(parent_id) = env.get_parent() {
            self.set_variable(parent_id, symbol_id, value)
        } else {
            Error::generic_execution_error(
                "Cannot find a variable to set."
            ).into_result()
        }
    }

    pub fn set_function(
        &mut self,
        id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), Error> {
        let env = self.get_mut(id)?;

        if env.has_function(symbol_id) {
            env.set_function(symbol_id, value)
        } else if let Some(parent_id) = env.get_parent() {
            self.set_function(parent_id, symbol_id, value)
        } else {
            Error::generic_execution_error(
                "Cannot find a function to set."
            ).into_result()
        }
    }

    pub fn lookup_environment_by_variable(
        &self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId
    ) -> Result<Option<EnvironmentId>, Error> {
        let env = self.get(environment_id)?;

        match env.has_variable(variable_symbol_id) {
            true => Ok(Some(environment_id)),
            false => match env.get_parent() {
                Some(parent) => self.lookup_environment_by_variable(parent, variable_symbol_id),
                None => Ok(None)
            }
        }
    }

    pub fn lookup_environment_by_function(
        &self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId
    ) -> Result<Option<EnvironmentId>, Error> {
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

    #[cfg(test)]
    mod free_environment {
        use super::*;

        #[test]
        fn removes_correctly() {
            let mut arena = EnvironmentArena::new();
            let env_id = arena.alloc();

            arena.get(env_id).unwrap();
            arena.free_environment(env_id).unwrap();
            arena.get(env_id).err().unwrap();
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod define_variable__define_function {
        use super::*;

        #[test]
        fn define_works_correctly_for_a_single_environment() {
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
        fn cannot_define_variable_twice() {
            let mut arena = EnvironmentArena::new();

            let env_id = arena.alloc();
            let key = SymbolId::new(0);

            arena.define_variable(env_id, key, Value::Integer(1)).unwrap();
            assert!(arena.define_variable(env_id, key, Value::Integer(1)).is_err());
        }

        #[test]
        fn cannot_define_function_twice() {
            let mut arena = EnvironmentArena::new();

            let env_id = arena.alloc();
            let key = SymbolId::new(0);

            arena.define_function(env_id, key, Value::Integer(1)).unwrap();
            assert!(arena.define_function(env_id, key, Value::Integer(1)).is_err());
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_variable__set_function {
        use super::*;

        #[test]
        fn set_works_correctly_for_a_single_environment() {
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
        fn when_defined_only_in_parent_set_only_in_parent() {
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

        #[test]
        fn cannot_set_to_not_defined_variable() {
            let mut arena = EnvironmentArena::new();

            let env_id = arena.alloc();
            let key = SymbolId::new(0);

            assert!(arena.set_variable(env_id, key, Value::Integer(2)).is_err());
        }

        #[test]
        fn cannot_set_to_not_defined_function() {
            let mut arena = EnvironmentArena::new();

            let env_id = arena.alloc();
            let key = SymbolId::new(0);

            assert!(arena.set_function(env_id, key, Value::Integer(2)).is_err());
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod lookup_variable__lookup_function {
        use super::*;

        #[test]
        fn lookups_from_parents_works_correctly() {
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
    }

    #[cfg(test)]
    mod lookup_environment_by_variable {
        use super::*;

        #[test]
        fn returns_current_environment_when_variable_is_defined_here() {
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
        fn returns_parent_environment_when_variable_is_defined_here() {
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
        fn returns_parent_environment_when_variable_is_defined_2() {
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
        fn returns_none_when_variable_is_defined_nowhere() {
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
