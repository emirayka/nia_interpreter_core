use crate::interpreter::object::{Object, ObjectId};
use crate::interpreter::symbol::Symbol;
use crate::interpreter::value::Value;

pub struct ObjectArena {
    objects: Vec<Object>,
}

impl ObjectArena {
    pub fn new() -> ObjectArena {
        ObjectArena {
            objects: Vec::new()
        }
    }
}

impl ObjectArena {
    pub fn make(&mut self) -> ObjectId {
        let object_id = ObjectId::new(self.objects.len());

        self.objects.push(Object::new());

        object_id
    }

    pub fn make_child(&mut self, prototype_id: ObjectId) -> ObjectId {
        let object_id = ObjectId::new(self.objects.len());

        self.objects.push(Object::new_child(prototype_id));

        object_id
    }

    pub fn get_object(&self, object_id: ObjectId) -> &Object {
        self.objects.get(object_id.get_index()).unwrap()
    }

    pub fn get_object_mut(&mut self, object_id: ObjectId) -> &mut Object {
        self.objects.get_mut(object_id.get_index()).unwrap()
    }

    pub fn get_item(&self, object_id: ObjectId, key: &Symbol) -> Option<&Value> {
        let object = self.get_object(object_id);

        match object.get_item(key) {
            Some(value) => Some(value),
            None => match object.get_prototype() {
                Some(prototype_id) => self.get_item(prototype_id, key),
                None => None
            }
        }
    }

    pub fn set_item(&mut self, object_id: ObjectId, key: &Symbol, value: Value) {
        let object = self.get_object_mut(object_id);

        object.set_item(key, value);
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

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod get_item__set_item {
        use super::*;

        #[test]
        fn able_to_set_and_get_value() {
            let mut arena = ObjectArena::new();

            let key = new_symbol("eh");
            let object_id = arena.make();

            arena.set_item(object_id, &key, Value::Integer(1));

            assert_eq!(Some(&Value::Integer(1)), arena.get_item(object_id, &key));
        }

        #[test]
        fn able_to_get_value_from_prototype() {
            let mut arena = ObjectArena::new();

            let key = new_symbol("eh");

            let prototype_id = arena.make();
            let child_id = arena.make_child(prototype_id);

            arena.set_item(prototype_id, &key, Value::Integer(1));

            assert_eq!(Some(&Value::Integer(1)), arena.get_item(child_id, &key));
        }

        #[test]
        fn does_not_set_value_to_prototype() {
            let mut arena = ObjectArena::new();

            let key = new_symbol("eh");

            let prototype_id = arena.make();
            let child_id = arena.make_child(prototype_id);

            arena.set_item(prototype_id, &key, Value::Integer(1));
            arena.set_item(child_id, &key, Value::Integer(2));

            assert_eq!(Some(&Value::Integer(1)), arena.get_item(prototype_id, &key));
            assert_eq!(Some(&Value::Integer(2)), arena.get_item(child_id, &key));
        }
    }

}