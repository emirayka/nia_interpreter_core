mod cons;
mod value;
mod environment;
mod environment_arena;
mod pre_reader;

use crate::interpreter::environment_arena::Arena;
use crate::interpreter::environment::ContestantId;

pub struct Interpreter {
    arena: Arena,
    root_env_id: ContestantId
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut arena = Arena::new();
        let root_env_id = arena.alloc();

        Interpreter {
            arena,
            root_env_id
        }
    }
}

//fn execute_string_element(string_element: &StringElement) -> Option<Value> {
//    let cloned_string = string.get_value().clone();
//    let value = Value::String(cloned_string);
//
//    Some(value)
//}
//
//fn execute_element(element: &Element) -> Option<Value> {
//    use Element::*;
//
//    match element {
//        String(string_element) => execute_string_element(string_element),
//        _ => unimplemented!()
//    }
//}

//impl Interpreter {
//    pub fn execute(element: &Element) -> Option<Value> {
//        execute_element(element)
//    }
//}