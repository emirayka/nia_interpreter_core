use std::collections::HashMap;
use crate::interpreter::function::Function;
use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId {
    id: usize,
}

impl FunctionId {
    pub fn new(id: usize) -> FunctionId {
        FunctionId {
            id
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

pub struct FunctionArena {
    arena: HashMap<FunctionId, Function>,
    next_id: usize,
}

impl FunctionArena {
    pub fn new() -> FunctionArena {
        FunctionArena {
            arena: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn register_function(&mut self, func: Function) -> FunctionId {
        let function_id = FunctionId::new(self.next_id);

        self.arena.insert(function_id, func);
        self.next_id += 1;

        function_id
    }

    pub fn get_function(&self, function_id: FunctionId) -> Result<&Function, Error> {
        self.arena
            .get(&function_id)
            .ok_or(Error::failure(
                format!("Cannot get a function with id: {}", function_id.get_id())
            ))
    }

    pub fn free_function(&mut self, function_id: FunctionId) -> Result<(), Error> {
        match self.arena.remove(&function_id) {
            Some(_) => Ok(()),
            _ => Error::failure(
                format!("Cannot get a function with id: {}", function_id.get_id())
            ).into_result()
        }
    }

    pub fn get_all_function_identifiers(&self) -> Vec<FunctionId> {
        let mut result = Vec::new();

        for k in self.arena.keys() {
            result.push(*k)
        }

        result
    }

    pub fn get_gc_items(&self, function_id: FunctionId) -> Result<Option<Vec<Value>>, Error> {
        match self.arena.get(&function_id) {
            Some(function) => Ok(function.get_gc_items()),
            _ => Error::failure(
                format!("Cannot get a function with id: {}", function_id.get_id())
            ).into_result()
        }
    }

    pub fn get_gc_environment(&self, function_id: FunctionId) -> Result<Option<EnvironmentId>, Error> {
        match self.arena.get(&function_id) {
            Some(function) => Ok(function.get_gc_environment()),
            _ => Error::failure(
                format!("Cannot get a function with id: {}", function_id.get_id())
            ).into_result()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::interpreter::Interpreter;
    use crate::interpreter::environment::EnvironmentId;
    use crate::interpreter::value::Value;

    fn test_func(
        _interpreter: &mut Interpreter,
        _environment_id: EnvironmentId,
        _values: Vec<Value>
    ) -> Result<Value, Error> {
        Ok(Value::Integer(1))
    }

    #[cfg(test)]
    mod free_function {
        use super::*;
        use crate::interpreter::function::BuiltinFunction;

        #[test]
        fn frees_function() {
            let mut function_arena = FunctionArena::new();

            let function = Function::Builtin(BuiltinFunction::new(test_func));
            let function_id = function_arena.register_function(function);

            assert!(function_arena.get_function(function_id).is_ok());
            assert!(function_arena.free_function(function_id).is_ok());
            assert!(function_arena.get_function(function_id).is_err());
        }

        #[test]
        fn returns_err_when_cannot_find_a_function() {
            let mut function_arena = FunctionArena::new();

            let function_id = FunctionId::new(234234);

            assert!(function_arena.free_function(function_id).is_err());
        }

        #[test]
        fn returns_err_when_attempts_to_free_function_twice() {
            let mut function_arena = FunctionArena::new();

            let function = Function::Builtin(BuiltinFunction::new(test_func));
            let function_id = function_arena.register_function(function);

            assert!(function_arena.free_function(function_id).is_ok());
            assert!(function_arena.free_function(function_id).is_err());
        }
    }
}

