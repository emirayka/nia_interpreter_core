use std::collections::HashMap;
use crate::interpreter::keyword::keyword::Keyword;

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

    pub fn get_keyword(&self, keyword_id: KeywordId) -> Result<&Keyword, ()> {
        self.arena.get(&keyword_id).ok_or(())
    }

    pub fn intern_keyword(&mut self, keyword_name: String) -> KeywordId {
        if self.mapping.contains_key(&keyword_name) {
            *self.mapping.get(&keyword_name).unwrap()
        } else {
            self.make_keyword(keyword_name)
        }
    }
}

// todo: arena tests