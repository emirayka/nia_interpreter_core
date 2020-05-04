#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Symbol {
    name: String,
    gensym_id: usize,
}

impl Symbol {
    pub fn new(name: String, counter: usize) -> Symbol {
        Symbol {
            name,
            gensym_id: counter,
        }
    }

    pub fn from(name: &str) -> Symbol {
        Symbol {
            name: String::from(name),
            gensym_id: 0,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_gensym_id(&self) -> usize {
        self.gensym_id
    }
}
