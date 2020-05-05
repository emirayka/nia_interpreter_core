use std::collections::HashMap;

use crate::interpreter::error::Error;
use crate::interpreter::value::NiaString;
use crate::interpreter::value::StringId;

#[derive(Clone)]
pub struct StringArena {
    arena: HashMap<StringId, NiaString>,
    mapping: HashMap<String, StringId>,
    next_id: usize,
}

impl StringArena {
    pub fn new() -> StringArena {
        StringArena {
            arena: HashMap::new(),
            mapping: HashMap::new(),
            next_id: 0,
        }
    }

    fn make_string(&mut self, s: &str) -> StringId {
        let string = NiaString::new(String::from(s));
        let string_id = StringId::new(self.next_id);

        self.arena.insert(string_id, string);
        self.mapping.insert(String::from(s), string_id);
        self.next_id += 1;

        string_id
    }

    pub fn intern_string(&mut self, string_name: &str) -> StringId {
        if self.mapping.contains_key(string_name) {
            *self.mapping.get(string_name).unwrap()
        } else {
            self.make_string(string_name)
        }
    }

    pub fn get_string(&self, string_id: StringId) -> Result<&NiaString, Error> {
        self.arena.get(&string_id).ok_or(Error::failure(format!(
            "Cannot find a string with id: {}",
            string_id.get_id()
        )))
    }

    pub fn free_string(&mut self, string_id: StringId) -> Result<(), Error> {
        let string = match self.arena.remove(&string_id) {
            Some(hia_string) => hia_string,
            _ => {
                return Error::failure(format!(
                    "Cannot find a string with id: {}",
                    string_id.get_id()
                ))
                .into();
            },
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

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[cfg(test)]
    mod make_string {
        use super::*;

        #[test]
        fn makes_string_in_arena() {
            let mut string_arena = StringArena::new();

            let expected = "String";
            let string_id = string_arena.make_string(expected);

            let result =
                string_arena.get_string(string_id).unwrap().get_string();

            nia_assert_equal(expected, result);
        }

        #[test]
        fn works_twice() {
            let mut string_arena = StringArena::new();

            let expected = "String";
            string_arena.make_string(expected);
            let string_id = string_arena.make_string(expected);

            let result =
                string_arena.get_string(string_id).unwrap().get_string();

            nia_assert_equal(expected, result);
        }
    }

    #[cfg(test)]
    mod intern {
        use super::*;

        #[test]
        fn interns_string() {
            let mut string_arena = StringArena::new();

            let expected = "String";
            let string_id = string_arena.intern_string(expected);

            let result =
                string_arena.get_string(string_id).unwrap().get_string();

            nia_assert_equal(expected, result);
        }
    }

    #[cfg(test)]
    mod free {
        use super::*;

        #[test]
        fn frees_string() {
            let mut string_arena = StringArena::new();

            let string = "string";
            let string_id = string_arena.intern_string(string);

            nia_assert(string_arena.get_string(string_id).is_ok());
            nia_assert(string_arena.free_string(string_id).is_ok());
            nia_assert(string_arena.get_string(string_id).is_err());

            nia_assert(!string_arena.arena.contains_key(&string_id));
            nia_assert(!string_arena.mapping.contains_key(string));
        }

        #[test]
        fn returns_error_when_cannot_find_string_with_provided_id() {
            let mut string_arena = StringArena::new();

            let string_id = StringId::new(9994);

            nia_assert(string_arena.free_string(string_id).is_err());
        }

        #[test]
        fn returns_error_when_freed_twice() {
            let mut string_arena = StringArena::new();

            let string_id = string_arena.intern_string("string");

            string_arena.free_string(string_id).unwrap();
            nia_assert(string_arena.free_string(string_id).is_err());
        }
    }
}
