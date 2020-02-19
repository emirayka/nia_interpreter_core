use std::collections::HashMap;
use crate::interpreter::string::string::VString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringId {
    id: usize
}

impl StringId {
    pub fn new(id: usize) -> StringId {
        StringId {
            id
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

pub struct StringArena {
    arena: HashMap<StringId, VString>,
    mapping: HashMap<String, StringId>,
    next_id: usize,
}

impl StringArena {
    pub fn new() -> StringArena {
        StringArena {
            arena: HashMap::new(),
            mapping: HashMap::new(),
            next_id: 0
        }
    }

    pub fn make_string(&mut self, string: String) -> StringId {
        let string = VString::new(string);
        let string_id = StringId::new(self.next_id);

        self.arena.insert(string_id, string);
        self.next_id += 1;

        string_id
    }

    pub fn get_string(&self, string_id: StringId) -> Option<&VString> {
        self.arena.get(&string_id)
    }


    pub fn intern_string(&mut self, string_name: String) -> StringId {
        if self.mapping.contains_key(&string_name) {
            *self.mapping.get(&string_name).unwrap()
        } else {
            self.make_string(string_name)
        }
    }
}
