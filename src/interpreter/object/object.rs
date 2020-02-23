use std::collections::HashMap;

use crate::interpreter::symbol::SymbolId;
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
    items: HashMap<SymbolId, Value>,
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

    pub fn set_prototype(&mut self, object_id: ObjectId) {
        self.prototype = Some(object_id)
    }

    pub fn has_item(&self, symbol_id: SymbolId) -> bool {
        self.items.contains_key(&symbol_id)
    }

    pub fn get_item(&self, symbol_id: SymbolId) -> Option<Value> {
        match self.items.get(&symbol_id) {
            Some(v) => Some(*v),
            None => None
        }
    }

    pub fn set_item(&mut self, symbol_id: SymbolId, value: Value) {
        if self.items.contains_key(&symbol_id) {
            *self.items.get_mut(&symbol_id).unwrap() = value;
        } else {
            self.items.insert(symbol_id, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_and_sets_items() {
        let mut object = Object::new();

        let key = SymbolId::new(0);
        let value = Value::Integer(1);

        object.set_item(key, value);

        assert_eq!(Value::Integer(1), object.get_item(key).unwrap());
    }

    #[test]
    fn able_to_set_value_again() {
        let mut object = Object::new();

        let key = SymbolId::new(0);
        let value1 = Value::Integer(1);
        let value2 = Value::Integer(2);

        object.set_item(key, value1);
        object.set_item(key, value2);

        assert_eq!(Value::Integer(2), object.get_item(key).unwrap());
    }

    #[test]
    fn returns_none_if_no_value_was_set() {
        let object = Object::new();

        let key = SymbolId::new(0);

        assert_eq!(None, object.get_item(key));
    }
}

