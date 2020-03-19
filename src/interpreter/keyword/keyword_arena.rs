use std::collections::HashMap;
use crate::interpreter::keyword::Keyword;
use crate::interpreter::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeywordId {
    id: usize
}

impl KeywordId {
    pub fn new(id: usize) -> KeywordId {
        KeywordId {
            id
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

pub struct KeywordArena {
    arena: HashMap<KeywordId, Keyword>,
    mapping: HashMap<String, KeywordId>,
    next_id: usize,
}

impl KeywordArena {
    pub fn new() -> KeywordArena {
        KeywordArena {
            arena: HashMap::new(),
            mapping: HashMap::new(),
            next_id: 0
        }
    }

    pub fn make_keyword(&mut self, keyword_name: String) -> KeywordId {
        let keyword = Keyword::new(keyword_name.clone());
        let keyword_id = KeywordId::new(self.next_id);

        self.arena.insert(keyword_id, keyword);
        self.mapping.insert(keyword_name, keyword_id);
        self.next_id += 1;

        keyword_id
    }

    pub fn get_keyword(&self, keyword_id: KeywordId) -> Result<&Keyword, Error> {
        self.arena
            .get(&keyword_id)
            .ok_or(Error::failure(
                format!("Cannot find a keyword with id: {}", keyword_id.get_id())
            ))
    }

    pub fn intern_keyword(&mut self, keyword_name: String) -> KeywordId {
        if self.mapping.contains_key(&keyword_name) {
            *self.mapping.get(&keyword_name).unwrap()
        } else {
            self.make_keyword(keyword_name)
        }
    }

    pub fn free_keyword(&mut self, keyword_id: KeywordId) -> Result<(), Error> {
        let keyword = match self.arena.remove(&keyword_id) {
            Some(keyword) => keyword,
            _ => return Error::failure(
                format!("Cannot find a keyword with id: {}", keyword_id.get_id())
            ).into_result()
        };

        self.arena.remove(&keyword_id);
        self.mapping.remove(keyword.get_name());

        Ok(())
    }

    pub fn get_all_keyword_identifiers(&self) -> Vec<KeywordId> {
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
    mod free_keyword {
        use super::*;

        #[test]
        fn frees_keyword() {
            let mut keyword_arena = KeywordArena::new();

            let expected = "keyword";
            let keyword_id = keyword_arena.intern_keyword(String::from(expected));

            assert!(keyword_arena.get_keyword(keyword_id).is_ok());
            assert!(keyword_arena.free_keyword(keyword_id).is_ok());
            assert!(keyword_arena.get_keyword(keyword_id).is_err());

            assert!(!keyword_arena.arena.contains_key(&keyword_id));
            assert!(!keyword_arena.mapping.contains_key(expected));
        }

        #[test]
        fn returns_failure_when_attempts_to_free_keyword_with_unknown_id() {
            let mut keyword_arena = KeywordArena::new();

            let expected = "";
            let keyword_id = KeywordId::new(23444);

            assert!(keyword_arena.free_keyword(keyword_id).is_err());
        }

        #[test]
        fn returns_failure_when_attempts_to_free_keyword_twice() {
            let mut keyword_arena = KeywordArena::new();

            let expected = "";
            let keyword_id = keyword_arena.intern_keyword(String::from(expected));

            assert!(keyword_arena.free_keyword(keyword_id).is_ok());
            assert!(keyword_arena.free_keyword(keyword_id).is_err());
        }
    }
}
