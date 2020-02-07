use std::collections::HashMap;

use crate::interpreter::symbol::Symbol;
use crate::interpreter::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId {
    index: usize
}

impl ObjectId {
    pub fn new(index: usize) -> ObjectId {
        ObjectId {
            index
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object {
    items: HashMap<Symbol, Value>,
    prototype: Option<ObjectId>,
}

impl Object {
    pub fn new() -> Object {
        Object {
            items: HashMap::new(),
            prototype: None,
        }
    }

    pub fn new_child(object_id: ObjectId) -> Object {
        Object {
            items: HashMap::new(),
            prototype: Some(object_id),
        }
    }

    pub fn get_prototype(&self) -> Option<ObjectId> {
        self.prototype
    }

    pub fn has_item(&self, symbol: &Symbol) -> bool {
        self.items.contains_key(symbol)
    }

    pub fn get_item(&self, symbol: &Symbol) -> Option<&Value> {
        self.items.get(symbol)
    }

    pub fn set_item(&mut self, symbol: &Symbol, value: Value) {
        if self.items.contains_key(symbol) {
            *self.items.get_mut(symbol).unwrap() = value;
        } else {
            self.items.insert(symbol.clone(), value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::symbol::SymbolArena;

    fn new_symbol(name: &str) -> Symbol {
        let mut symbol_arena = SymbolArena::new();

        symbol_arena.intern(name)
    }

    #[test]
    fn gets_and_sets_items() {
        let mut object = Object::new();

        let key = new_symbol("key");
        let value = Value::Integer(1);

        object.set_item(&key, value);

        assert_eq!(Some(&Value::Integer(1)), object.get_item(&key));
    }

    #[test]
    fn able_to_set_value_again() {
        let mut object = Object::new();

        let key = new_symbol("key");
        let value1 = Value::Integer(1);
        let value2 = Value::Integer(2);

        object.set_item(&key, value1);
        object.set_item(&key, value2);

        assert_eq!(Some(&Value::Integer(2)), object.get_item(&key));
    }

    #[test]
    fn returns_none_if_no_value_was_set() {
        let object = Object::new();

        let key = new_symbol("key");

        assert_eq!(None, object.get_item(&key));
    }
}

