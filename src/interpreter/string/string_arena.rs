use std::collections::HashMap;
use crate::interpreter::string::VString;
use crate::interpreter::error::Error;

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
        let vstring = VString::new(string.clone());
        let string_id = StringId::new(self.next_id);

        self.arena.insert(string_id, vstring);
        self.mapping.insert(string, string_id);
        self.next_id += 1;

        string_id
    }

    pub fn get_string(&self, string_id: StringId) -> Result<&VString, Error> {
        self.arena
            .get(&string_id)
            .ok_or(Error::failure(
                format!("Cannot find a string with id: {}", string_id.get_id())
            ))
    }

    pub fn intern_string(&mut self, string_name: String) -> StringId {
        if self.mapping.contains_key(&string_name) {
            *self.mapping.get(&string_name).unwrap()
        } else {
            self.make_string(string_name)
        }
    }

    pub fn free_string(&mut self, string_id: StringId) -> Result<(), Error> {
        let string = match self.arena.remove(&string_id) {
            Some(vstring) => {
                vstring
            },
            _ => return Error::failure(
                format!("Cannot find a string with id: {}", string_id.get_id())
            ).into_result()
        };

        self.mapping.remove(string.get_string());

        Ok(())
    }

    pub fn get_all_string_identifiers(&self) -> Vec<StringId> {
        let mut result = Vec::new();

        for k in self.arena.keys() {
            result.push(*k)
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod make_string {
        use super::*;

        #[test]
        fn makes_string_in_arena() {
            let mut string_arena = StringArena::new();

            let expected = "String";
            let string_id = string_arena.make_string(String::from(expected));

            let result = string_arena.get_string(string_id).unwrap().get_string();

            assert_eq!(expected, result);
        }

        #[test]
        fn works_twice() {
            let mut string_arena = StringArena::new();

            let expected = "String";
            let string_id = string_arena.make_string(String::from(expected));
            let string_id = string_arena.make_string(String::from(expected));

            let result = string_arena.get_string(string_id).unwrap().get_string();

            assert_eq!(expected, result);
        }
    }

    #[cfg(test)]
    mod intern {
        use super::*;

        #[test]
        fn interns_string() {
            let mut string_arena = StringArena::new();

            let expected = "String";
            let string_id = string_arena.intern_string(String::from(expected));

            let result = string_arena.get_string(string_id).unwrap().get_string();

            assert_eq!(expected, result);
        }
    }

    #[cfg(test)]
    mod free {
        use super::*;

        #[test]
        fn frees_string() {
            let mut string_arena = StringArena::new();

            let string = "string";
            let string_id = string_arena.intern_string(String::from(string));

            assert!(string_arena.get_string(string_id).is_ok());
            assert!(string_arena.free_string(string_id).is_ok());
            assert!(string_arena.get_string(string_id).is_err());

            assert!(!string_arena.arena.contains_key(&string_id));
            assert!(!string_arena.mapping.contains_key(string));
        }

        #[test]
        fn returns_error_when_cannot_find_string_with_provided_id() {
            let mut string_arena = StringArena::new();

            let string_id = StringId::new(9994);

            assert!(string_arena.free_string(string_id).is_err());
        }

        #[test]
        fn returns_error_when_freed_twice() {
            let mut string_arena = StringArena::new();

            let string_id = string_arena.intern_string(String::from("arst"));

            string_arena.free_string(string_id).unwrap();
            assert!(string_arena.free_string(string_id).is_err());
        }
    }
}
