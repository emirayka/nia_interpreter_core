use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeywordId {
    id: usize,
}

impl KeywordId {
    pub fn new(id: usize) -> KeywordId {
        KeywordId { id }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}
impl fmt::Display for KeywordId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
