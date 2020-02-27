use std::hash::Hash;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId {
    id: usize
}

impl SymbolId {
    pub fn new(id: usize) -> SymbolId {
        SymbolId {
            id
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Symbol {
    name: String,
    gensym_id: usize
}

impl Symbol {
    fn new(name: String, counter: usize) -> Symbol {
        Symbol {
            name,
            gensym_id: counter
        }
    }

    fn from(name: &str) -> Symbol {
        Symbol {
            name: String::from(name),
            gensym_id: 0
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_gensym_id(&self) -> usize {
        self.gensym_id
    }

    pub fn is_nil(&self) -> bool {
        &self.name == "nil"
    }
}

#[derive(Debug)]
pub struct SymbolArena {
    symbols: HashMap<SymbolId, Symbol>,
    mapping: HashMap<String, Vec<SymbolId>>,
    next_id: usize,
}

impl SymbolArena {
    pub fn new() -> SymbolArena {
        SymbolArena {
            symbols: HashMap::new(),
            mapping: HashMap::new(),
            next_id: 0,
        }
    }

    fn ensure_symbol_defined(&mut self, symbol_name: &str) {
        match self.mapping.get_mut(symbol_name) {
            Some(_) => (),
            None => {
                let vector = Vec::new();

                self.mapping.insert(String::from(symbol_name), vector);
            }
        };
    }

    fn ensure_symbol_internable(&mut self, symbol_name: &str) {
        match self.mapping.get_mut(symbol_name) {
            Some(vector) => {
                match vector.get(0) {
                    Some(_) => (),
                    None => {
                        let symbol = Symbol::from(symbol_name);
                        let symbol_id = SymbolId::new(self.next_id);

                        self.next_id += 1;

                        vector.push(symbol_id);
                        self.symbols.insert(symbol_id, symbol);
                    }
                }
            },
            None => {
                self.ensure_symbol_defined(symbol_name);
                self.ensure_symbol_internable(symbol_name);
            }
        };
    }

    pub fn get_symbol(&self, symbol_id: SymbolId) -> Result<&Symbol, ()> {
        self.symbols.get(&symbol_id).ok_or(())
    }

    pub fn intern(&mut self, symbol_name: &str) -> SymbolId {
        self.ensure_symbol_defined(symbol_name);
        self.ensure_symbol_internable(symbol_name);

        match self.mapping.get(symbol_name) {
            Some(symbols) => symbols[0],
            _ => unreachable!()
        }
    }

    pub fn gensym(&mut self, symbol_name: &str) -> SymbolId {
        self.ensure_symbol_defined(symbol_name);
        self.ensure_symbol_internable(symbol_name);

        match self.mapping.get_mut(symbol_name) {
            Some(symbols) => {
                let counter = symbols.len();
                let symbol = Symbol::new(String::from(symbol_name), counter);
                let symbol_id = SymbolId::new(self.next_id);

                self.next_id += 1;

                symbols.push(symbol_id);
                self.symbols.insert(symbol_id, symbol);

                symbols[counter]
            },
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_interns_correctly() {
        let mut arena = SymbolArena::new();

        let sym1 = arena.intern("test");
        let sym2 = arena.intern("test");

        assert_eq!(SymbolId::new(0), sym1);
        assert_eq!(SymbolId::new(0), sym2);
    }

    #[test]
    pub fn test_gensyms_correctly() {
        let mut arena = SymbolArena::new();

        let sym = arena.intern("test");
        let sym1 = arena.gensym("test");
        let sym2 = arena.gensym("test");

        assert_ne!(sym, sym1);
        assert_ne!(sym, sym2);
        assert_ne!(sym1, sym2);

        assert_eq!(SymbolId::new(0), sym);
        assert_eq!(SymbolId::new(1), sym1);
        assert_eq!(SymbolId::new(2), sym2);
    }
}
