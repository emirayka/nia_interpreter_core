use std::hash::Hash;
use std::collections::HashMap;
use crate::interpreter::error::Error;
use crate::interpreter::value::Value;

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

    pub fn to_value(&self) -> Value {
        Value::Symbol(*self)
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
}

#[derive(Debug)]
pub struct SymbolArena {
    arena: HashMap<SymbolId, Symbol>,
    mapping: HashMap<String, Vec<SymbolId>>,
    next_id: usize,
}

impl SymbolArena {
    pub fn new() -> SymbolArena {
        SymbolArena {
            arena: HashMap::new(),
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
                        self.arena.insert(symbol_id, symbol);
                    }
                }
            },
            None => {
                self.ensure_symbol_defined(symbol_name);
                self.ensure_symbol_internable(symbol_name);
            }
        };
    }

    pub fn get_symbol(&self, symbol_id: SymbolId) -> Result<&Symbol, Error> {
        self.arena
            .get(&symbol_id)
            .ok_or(Error::failure(
                format!("Cannot find a symbol with id: {}", symbol_id.get_id())
            ))
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
                self.arena.insert(symbol_id, symbol);

                symbols[counter]
            },
            _ => unreachable!()
        }
    }

    pub fn free_symbol(&mut self, symbol_id: SymbolId) -> Result<(), Error> {
        let symbol = match self.arena.remove(&symbol_id) {
            Some(symbol) => symbol,
            _ => return Error::failure(
                format!("Cannot find a symbol with id: {}", symbol_id.get_id())
            ).into_result()
        };

        self.mapping.remove(symbol.get_name());

        Ok(())
    }

    pub fn get_all_symbol_identifiers(&self) -> Vec<SymbolId> {
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
    mod intern {
        use super::*;

        #[test]
        pub fn interns_correctly() {
            let mut arena = SymbolArena::new();

            let sym1 = arena.intern("test");
            let sym2 = arena.intern("test");

            assert_eq!(SymbolId::new(0), sym1);
            assert_eq!(SymbolId::new(0), sym2);
        }
    }

    #[cfg(test)]
    mod gensym {
        use super::*;

        #[test]
        pub fn gensyms_correctly() {
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

    #[cfg(test)]
    mod free_symbol {
        use super::*;

        #[test]
        fn frees_symbol() {
            let mut symbol_arena = SymbolArena::new();

            let name = "symbol";
            let symbol_id = symbol_arena.intern(name);

            assert!(symbol_arena.get_symbol(symbol_id).is_ok());
            assert!(symbol_arena.free_symbol(symbol_id).is_ok());
            assert!(symbol_arena.get_symbol(symbol_id).is_err());

            assert!(!symbol_arena.arena.contains_key(&symbol_id));
            assert!(!symbol_arena.mapping.contains_key(name));
        }

        #[test]
        fn returns_failure_when_attempts_to_free_a_symbol_with_unknown_id() {
            let mut symbol_arena = SymbolArena::new();

            let symbol_id = SymbolId::new(23234234);

            assert!(symbol_arena.free_symbol(symbol_id).is_err());
        }

        #[test]
        fn returns_failure_when_attempts_to_free_a_symbol_twice() {
            let mut symbol_arena = SymbolArena::new();

            let name = "symbol";
            let symbol_id = symbol_arena.intern(name);

            assert!(symbol_arena.free_symbol(symbol_id).is_ok());
            assert!(symbol_arena.free_symbol(symbol_id).is_err());
        }
    }

}
