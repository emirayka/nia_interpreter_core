use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct Symbol {
    name: String,
    counter: i32
}

impl Symbol {
    pub fn new(name: String, count: i32) -> Symbol {
        Symbol {
            name,
            counter: count
        }
    }

    pub fn from(name: &str) -> Symbol {
        Symbol {
            name: name.to_string(),
            counter: 0
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_counter(&self) -> i32 {
        self.counter
    }

    pub fn is_nil(&self) -> bool {
        &self.name == "nil"
    }
}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.counter.hash(state);
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
            self.counter == other.counter
    }
}

impl Eq for Symbol {
}
