use std::hash::Hash;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Symbol {
    name: String,
    counter: usize
}

impl Symbol {
    fn new(name: String, counter: usize) -> Symbol {
        Symbol {
            name,
            counter
        }
    }

    fn from(name: &str) -> Symbol {
        Symbol {
            name: name.to_string(),
            counter: 0
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_counter(&self) -> usize {
        self.counter
    }

    pub fn is_nil(&self) -> bool {
        &self.name == "nil"
    }
}

#[derive(Debug)]
pub struct SymbolArena {
    symbols: HashMap<String, Vec<Symbol>>,
}

impl SymbolArena {
    pub fn new() -> SymbolArena {
        SymbolArena {
            symbols: HashMap::new(),
        }
    }

    fn ensure_symbol_defined(&mut self, symbol_name: &str) {
        match self.symbols.get_mut(symbol_name) {
            Some(_) => (),
            None => {
                let vector = Vec::new();

                self.symbols.insert(String::from(symbol_name), vector);
            }
        };
    }

    fn ensure_symbol_internable(&mut self, symbol_name: &str) {
        match self.symbols.get_mut(symbol_name) {
            Some(vector) => {
                match vector.get(0) {
                    Some(_) => (),
                    None => {
                        let symbol = Symbol::from(symbol_name);

                        vector.push(symbol);
                    }
                }
            },
            None => {
                self.ensure_symbol_defined(symbol_name);
                self.ensure_symbol_internable(symbol_name);
            }
        };
    }

    pub fn intern(&mut self, symbol_name: &str) -> Symbol {
        self.ensure_symbol_defined(symbol_name);
        self.ensure_symbol_internable(symbol_name);

        match self.symbols.get(symbol_name) {
            Some(symbols) => symbols[0].clone(),
            _ => unreachable!()
        }
    }

    pub fn gensym(&mut self, symbol_name: &str) -> Symbol {
        self.ensure_symbol_defined(symbol_name);
        self.ensure_symbol_internable(symbol_name);

        match self.symbols.get_mut(symbol_name) {
            Some(symbols) => {
                let counter = symbols.len();

                let symbol = Symbol::new(String::from(symbol_name), counter);
                symbols.push(symbol);

                symbols[counter].clone()
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

        assert_eq!(sym1, sym2);

        assert_eq!("test", &sym1.name);
        assert_eq!(0usize, sym1.counter);

        assert_eq!("test", &sym2.name);
        assert_eq!(0usize, sym2.counter);
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

        assert_eq!("test", &sym.name);
        assert_eq!(0usize, sym.counter);

        assert_eq!("test", &sym1.name);
        assert_eq!(1usize, sym1.counter);

        assert_eq!("test", &sym2.name);
        assert_eq!(2usize, sym2.counter);
    }
}
