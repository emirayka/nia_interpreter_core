#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keyword {
    keyword: String,
}

impl Keyword {
    pub fn new(keyword_name: String) -> Keyword {
        Keyword {
            keyword: keyword_name,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.keyword
    }

    pub fn is_const(&self) -> bool {
        self.keyword == "const"
    }
}
