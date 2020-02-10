use std::collections::HashMap;
use crate::interpreter::function::Function;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId {
    id: usize
}

impl FunctionId {
    pub fn new(id: usize) -> FunctionId {
        FunctionId {
            id
        }
    }
}

pub struct FunctionArena {
    arena: HashMap<FunctionId, Function>,
    next_id: usize
}

impl FunctionArena {
    pub fn new(id: usize) -> FunctionId {
        FunctionId {
            id
        }
    }

    pub fn register_function(&mut self, func: Function) -> FunctionId {
        let function_id = FunctionId::new(self.next_id);

        self.arena.insert(function_id, func);
        self.next_id += 1;

        function_id
    }
}