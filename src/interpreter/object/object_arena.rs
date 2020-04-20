use crate::interpreter::symbol::SymbolId;
use crate::interpreter::value::Value;
use crate::interpreter::object::{Object, ObjectId};
use nom::lib::std::collections::HashMap;
use crate::interpreter::error::Error;

#[derive(Clone)]
pub struct ObjectArena {
    arena: HashMap<ObjectId, Object>,
    next_id: usize,
}

impl ObjectArena {
    pub fn new() -> ObjectArena {
        ObjectArena {
            arena: HashMap::new(),
            next_id: 0
        }
    }
}

impl ObjectArena {
    pub fn make(&mut self) -> ObjectId {
        let id = self.next_id;
        let object_id = ObjectId::new(id);

        self.arena.insert(object_id, Object::new());
        self.next_id += 1;

        object_id
    }

    pub fn make_child(&mut self, prototype_id: ObjectId) -> ObjectId {
        let id = self.next_id;
        let object_id = ObjectId::new(id);

        self.arena.insert(object_id, Object::new_child(prototype_id));
        self.next_id += 1;

        object_id
    }

    pub fn get_object(&self, object_id: ObjectId) -> Result<&Object, Error> {
        self.arena
            .get(&object_id)
            .ok_or(Error::failure(
                format!("Cannot find an object with id: {}", object_id.get_id())
            ))
    }

    pub fn get_object_mut(&mut self, object_id: ObjectId) -> Result<&mut Object, Error> {
        self.arena
            .get_mut(&object_id)
            .ok_or(Error::failure(
                format!("Cannot find an object with id: {}", object_id.get_id())
            ))
    }

    pub fn free_object(&mut self, object_id: ObjectId) -> Result<(), Error> {
        match self.arena.remove(&object_id) {
            Some(_) => Ok(()),
            _ => Error::failure(
                format!("Cannot find an object with id: {}", object_id.get_id())
            ).into_result()
        }
    }

    pub fn get_all_object_identifiers(&self) -> Vec<ObjectId> {
        let mut result = Vec::new();

        for k in self.arena.keys() {
            result.push(*k)
        }

        result
    }

    pub fn get_item(&self, object_id: ObjectId, key: SymbolId) -> Result<Option<Value>, Error> {
        let object = self.get_object(object_id)?;

        match object.get_item(key) {
            Some(value) => Ok(Some(value)),
            None => match object.get_prototype() {
                Some(prototype_id) => self.get_item(prototype_id, key),
                None => Ok(None)
            }
        }
    }

    pub fn get_gc_items(&self, object_id: ObjectId) -> Result<Vec<Value>, Error> {
        match self.arena.get(&object_id) {
            Some(object) => Ok(object.get_gc_items()),
            _ => Error::failure(
                format!("Cannot find an object with id: {}", object_id.get_id())
            ).into_result()
        }
    }

    pub fn set_item(&mut self, object_id: ObjectId, key: SymbolId, value: Value) -> Result<(), Error> {
        let object = self.get_object_mut(object_id)?;

        object.set_item(key, value);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod get_item__set_item {
        use super::*;

        #[test]
        fn able_to_set_and_get_value() {
            let mut arena = ObjectArena::new();

            let key = SymbolId::new(0);
            let object_id = arena.make();

            arena.set_item(object_id, key, Value::Integer(1)).unwrap();

            assert_eq!(Value::Integer(1), arena.get_item(object_id, key).unwrap().unwrap());
        }

        #[test]
        fn able_to_get_value_from_prototype() {
            let mut arena = ObjectArena::new();

            let key = SymbolId::new(0);

            let prototype_id = arena.make();
            let child_id = arena.make_child(prototype_id);

            arena.set_item(prototype_id, key, Value::Integer(1)).unwrap();

            assert_eq!(Ok(Some(Value::Integer(1))), arena.get_item(child_id, key));
        }

        #[test]
        fn does_not_set_value_to_prototype() {
            let mut arena = ObjectArena::new();

            let key = SymbolId::new(0);

            let prototype_id = arena.make();
            let child_id = arena.make_child(prototype_id);

            arena.set_item(prototype_id, key, Value::Integer(1)).unwrap();
            arena.set_item(child_id, key, Value::Integer(2)).unwrap();

            assert_eq!(Ok(Some(Value::Integer(1))), arena.get_item(prototype_id, key));
            assert_eq!(Ok(Some(Value::Integer(2))), arena.get_item(child_id, key));
        }
    }

    #[cfg(test)]
    mod free_object {
        use super::*;

        #[test]
        fn frees_object() {
            let mut object_arena = ObjectArena::new();

            let object_id = object_arena.make();

            assert!(object_arena.get_object(object_id).is_ok());
            assert!(object_arena.free_object(object_id).is_ok());
            assert!(object_arena.get_object(object_id).is_err());
        }

        #[test]
        fn returns_failure_when_attempts_to_free_object_with_unknown_id() {
            let mut object_arena = ObjectArena::new();

            let object_id = ObjectId::new(234234);

            assert!(object_arena.free_object(object_id).is_err());
        }

        #[test]
        fn returns_failure_when_attempts_to_free_an_object_twice() {
            let mut object_arena = ObjectArena::new();

            let object_id = object_arena.make();

            assert!(object_arena.free_object(object_id).is_ok());
            assert!(object_arena.free_object(object_id).is_err());
        }
    }
}