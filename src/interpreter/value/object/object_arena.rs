use std::collections::HashMap;

use crate::interpreter::value::{Object, ObjectId, SymbolId, Value};

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
            next_id: 0,
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

        self.arena
            .insert(object_id, Object::new_child(prototype_id));
        self.next_id += 1;

        object_id
    }

    pub fn has_object(&self, object_id: ObjectId) -> bool {
        self.arena.contains_key(&object_id)
    }

    fn check_object_exists(&self, object_id: ObjectId) -> Result<(), Error> {
        if self.arena.contains_key(&object_id) {
            Ok(())
        } else {
            Error::failure(format!(
                "Cannot find an object with id: {}",
                object_id
            ))
            .into()
        }
    }

    pub fn get_object(&self, object_id: ObjectId) -> Result<&Object, Error> {
        self.arena.get(&object_id).ok_or(Error::failure(format!(
            "Cannot find an object with id: {}",
            object_id.get_id()
        )))
    }

    pub fn get_object_mut(
        &mut self,
        object_id: ObjectId,
    ) -> Result<&mut Object, Error> {
        self.arena.get_mut(&object_id).ok_or(Error::failure(format!(
            "Cannot find an object with id: {}",
            object_id.get_id()
        )))
    }

    pub fn free_object(&mut self, object_id: ObjectId) -> Result<(), Error> {
        match self.arena.remove(&object_id) {
            Some(_) => Ok(()),
            _ => Error::failure(format!(
                "Cannot find an object with id: {}",
                object_id.get_id()
            ))
            .into(),
        }
    }

    pub fn get_prototype(
        &self,
        object_id: ObjectId,
    ) -> Result<Option<ObjectId>, Error> {
        let object = self.get_object(object_id)?;

        Ok(object.get_prototype())
    }

    pub fn set_prototype(
        &mut self,
        child_id: ObjectId,
        prototype_id: ObjectId,
    ) -> Result<(), Error> {
        self.check_object_exists(prototype_id)?;

        let object = self.get_object_mut(child_id)?;

        object.set_prototype(prototype_id)
    }

    pub fn clear_prototype(
        &mut self,
        object_id: ObjectId,
    ) -> Result<(), Error> {
        let object = self.get_object_mut(object_id)?;

        object.clear_prototype()
    }

    pub fn has_property(
        &self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        let object = self.get_object(object_id)?;

        Ok(object.has_property(property_symbol_id))
    }

    pub fn get_property_value(
        &self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
    ) -> Result<Option<Value>, Error> {
        let object = self.get_object(object_id)?;

        match object.get_property_value(property_symbol_id)? {
            Some(value) => Ok(Some(value)),
            None => match object.get_prototype() {
                Some(prototype_id) => {
                    self.get_property_value(prototype_id, property_symbol_id)
                },
                None => Ok(None),
            },
        }
    }

    pub fn set_property(
        &mut self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        let object = self.get_object_mut(object_id)?;

        object.set_property(property_symbol_id, value)?;

        Ok(())
    }

    pub fn delete_property(
        &mut self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
    ) -> Result<(), Error> {
        let object = self.get_object_mut(object_id)?;

        object.delete_property(property_symbol_id)?;

        Ok(())
    }

    pub fn is_frozen(&self, object_id: ObjectId) -> Result<bool, Error> {
        let object = self.get_object(object_id)?;

        Ok(object.is_frozen())
    }

    pub fn freeze(&mut self, object_id: ObjectId) -> Result<(), Error> {
        let object = self.get_object_mut(object_id)?;

        object.freeze()?;

        Ok(())
    }

    pub fn get_property_flags(
        &self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
    ) -> Result<u8, Error> {
        let object = self.get_object(object_id)?;

        object.get_property_flags(property_symbol_id)
    }

    pub fn set_property_flags(
        &mut self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
        flags: u8,
    ) -> Result<(), Error> {
        let object = self.get_object_mut(object_id)?;

        object.set_property_flags(property_symbol_id, flags)
    }

    pub fn get_property_flag(
        &self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
        flag: u8,
    ) -> Result<bool, Error> {
        let object = self.get_object(object_id)?;

        object.get_property_flag(property_symbol_id, flag)
    }

    pub fn set_property_flag(
        &mut self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
        flag: u8,
        flag_value: bool,
    ) -> Result<(), Error> {
        let object = self.get_object_mut(object_id)?;

        object.set_property_flag(property_symbol_id, flag, flag_value)
    }

    pub fn is_property_internable(
        &self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        let object = self.get_object(object_id)?;

        object.is_property_internable(property_symbol_id)
    }

    pub fn is_property_writable(
        &self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        let object = self.get_object(object_id)?;

        object.is_property_writable(property_symbol_id)
    }

    pub fn is_property_enumerable(
        &self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        let object = self.get_object(object_id)?;

        object.is_property_enumerable(property_symbol_id)
    }

    pub fn is_property_configurable(
        &self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        let object = self.get_object(object_id)?;

        object.is_property_configurable(property_symbol_id)
    }

    pub fn set_property_internable(
        &mut self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
        value: bool,
    ) -> Result<(), Error> {
        let object = self.get_object_mut(object_id)?;

        object.set_property_internable(property_symbol_id, value)
    }

    pub fn set_property_writable(
        &mut self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
        value: bool,
    ) -> Result<(), Error> {
        let object = self.get_object_mut(object_id)?;

        object.set_property_writable(property_symbol_id, value)
    }

    pub fn set_property_enumerable(
        &mut self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
        value: bool,
    ) -> Result<(), Error> {
        let object = self.get_object_mut(object_id)?;

        object.set_property_enumerable(property_symbol_id, value)
    }

    pub fn set_property_configurable(
        &mut self,
        object_id: ObjectId,
        property_symbol_id: SymbolId,
        value: bool,
    ) -> Result<(), Error> {
        let object = self.get_object_mut(object_id)?;

        object.set_property_configurable(property_symbol_id, value)
    }

    pub fn get_all_object_identifiers(&self) -> Vec<ObjectId> {
        let mut result = Vec::new();

        for k in self.arena.keys() {
            result.push(*k)
        }

        result
    }

    pub fn get_gc_items(
        &self,
        object_id: ObjectId,
    ) -> Result<Vec<Value>, Error> {
        match self.arena.get(&object_id) {
            Some(object) => Ok(object.get_gc_items()),
            _ => Error::failure(format!(
                "Cannot find an object with id: {}",
                object_id.get_id()
            ))
            .into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    fn make_nonexistent_object_id() -> ObjectId {
        ObjectId::new(std::usize::MAX)
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod make__make_child__free_object {
        use super::*;

        #[test]
        fn makes_object() {
            let mut object_arena = ObjectArena::new();

            let object_id = object_arena.make();

            nia_assert_is_ok(&object_arena.get_object(object_id));
        }

        #[test]
        fn makes_child_object() {
            let mut object_arena = ObjectArena::new();

            let parent_id = object_arena.make();
            let child_id = object_arena.make_child(parent_id);

            nia_assert_is_ok(&object_arena.get_object(parent_id));
            nia_assert_is_ok(&object_arena.get_object(child_id));
            nia_assert_equal(
                Ok(Some(parent_id)),
                object_arena.get_prototype(child_id),
            );
        }

        #[test]
        fn frees_object() {
            let mut object_arena = ObjectArena::new();

            let object_id = object_arena.make();

            nia_assert_is_ok(&object_arena.get_object(object_id));
            nia_assert_is_ok(&object_arena.free_object(object_id));
            nia_assert_is_err(&object_arena.get_object(object_id));
        }

        #[test]
        fn returns_failure_when_attempts_to_free_object_with_unknown_id() {
            let mut object_arena = ObjectArena::new();

            let object_id = make_nonexistent_object_id();

            nia_assert_is_err(&object_arena.free_object(object_id));
        }

        #[test]
        fn returns_failure_when_attempts_to_free_an_object_twice() {
            let mut object_arena = ObjectArena::new();

            let object_id = object_arena.make();

            nia_assert_is_ok(&object_arena.free_object(object_id));
            nia_assert_is_err(&object_arena.free_object(object_id));
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod has_object__get_object__get_object_mut {
        use super::*;

        #[test]
        fn returns_object_reference() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            nia_assert_equal(true, arena.has_object(object_id));
            nia_assert_is_ok(&arena.get_object(object_id));
            nia_assert_is_ok(&arena.get_object_mut(object_id));
        }

        #[test]
        fn returns_failure_when_object_was_not_found() {
            let mut arena = ObjectArena::new();
            let object_id = make_nonexistent_object_id();

            nia_assert_equal(false, arena.has_object(object_id));
            nia_assert_is_err(&arena.get_object(object_id));
            nia_assert_is_err(&arena.get_object_mut(object_id));
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod is_frozen {
        use super::*;

        #[test]
        fn returns_false_when_object_is_not_frozen() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            nia_assert_equal(Ok(false), arena.is_frozen(object_id));
        }

        #[test]
        fn returns_true_when_object_is_frozen() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            nia_assert_is_ok(&arena.freeze(object_id));
            nia_assert_equal(Ok(true), arena.is_frozen(object_id));
        }

        #[test]
        fn returns_error_when_object_does_not_exist() {
            let mut arena = ObjectArena::new();
            let object_id = make_nonexistent_object_id();

            nia_assert_is_err(&arena.freeze(object_id));
            nia_assert_is_err(&arena.is_frozen(object_id));
        }
    }

    #[cfg(test)]
    mod freeze {
        use super::*;
        use crate::interpreter::value::value::Value::Symbol;

        #[test]
        fn freezes_object() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            nia_assert_equal(Ok(false), arena.is_frozen(object_id));
            nia_assert_is_ok(&arena.freeze(object_id));
            nia_assert_equal(Ok(true), arena.is_frozen(object_id));
        }

        #[test]
        fn returns_error_when_object_was_not_found() {
            let mut arena = ObjectArena::new();
            let object_id = make_nonexistent_object_id();

            nia_assert_is_err(&arena.freeze(object_id));
        }

        #[test]
        fn freezing_frozen_object_returns_error() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            nia_assert_equal(Ok(false), arena.is_frozen(object_id));

            nia_assert_is_ok(&arena.freeze(object_id));
            nia_assert_equal(Ok(true), arena.is_frozen(object_id));

            nia_assert_is_err(&arena.freeze(object_id));
            nia_assert_equal(Ok(true), arena.is_frozen(object_id));
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_prototype__clear_prototype__get_prototype {
        use super::*;

        #[test]
        fn sets_gets_and_clears_prototype() {
            let mut arena = ObjectArena::new();

            let parent_id = arena.make();
            let child_id = arena.make();

            nia_assert_equal(Ok(None), arena.get_prototype(child_id));
            nia_assert_equal(Ok(()), arena.set_prototype(child_id, parent_id));
            nia_assert_equal(
                Ok(Some(parent_id)),
                arena.get_prototype(child_id),
            );
            nia_assert_equal(Ok(()), arena.clear_prototype(child_id));
            nia_assert_equal(Ok(None), arena.get_prototype(child_id));
        }

        #[test]
        fn returns_error_when_child_is_frozen() {
            let mut arena = ObjectArena::new();

            let parent_id = arena.make();
            let child_id = arena.make();

            nia_assert_is_ok(&arena.freeze(child_id));
            nia_assert_is_err(&arena.set_prototype(child_id, parent_id));
        }

        #[test]
        fn returns_error_when_object_does_not_exist() {
            let mut arena = ObjectArena::new();

            let parent_id = arena.make();
            let child_id = make_nonexistent_object_id();

            nia_assert_is_err(&arena.get_prototype(child_id));
            nia_assert_is_err(&arena.set_prototype(child_id, parent_id));
            nia_assert_is_err(&arena.get_prototype(child_id));
            nia_assert_is_err(&arena.clear_prototype(child_id));
            nia_assert_is_err(&arena.get_prototype(child_id));
        }

        #[test]
        fn returns_error_when_prototype_does_not_exist() {
            let mut arena = ObjectArena::new();

            let child_id = arena.make();
            let parent_id = make_nonexistent_object_id();

            nia_assert_equal(Ok(None), arena.get_prototype(child_id));
            nia_assert_is_err(&arena.set_prototype(child_id, parent_id));
            nia_assert_equal(Ok(None), arena.get_prototype(child_id));
            nia_assert_equal(Ok(()), arena.clear_prototype(child_id));
            nia_assert_equal(Ok(None), arena.get_prototype(child_id));
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod has_property__delete_property_set_property__get_property {
        use super::*;

        #[test]
        fn gets_and_sets_value() {
            let mut arena = ObjectArena::new();

            let object_id = arena.make();
            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));

            nia_assert_equal(
                Ok(Some(value)),
                arena.get_property_value(object_id, property_symbol_id),
            );
        }

        #[test]
        fn gets_value_from_prototype_when_it_was_not_found_in_child() {
            let mut arena = ObjectArena::new();

            let prototype_id = arena.make();
            let child_id = arena.make_child(prototype_id);

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                prototype_id,
                property_symbol_id,
                value,
            ));
            nia_assert_equal(
                Ok(Some(value)),
                arena.get_property_value(child_id, property_symbol_id),
            );
        }

        #[test]
        fn does_not_set_value_to_prototype() {
            let mut arena = ObjectArena::new();

            let prototype_id = arena.make();
            let child_id = arena.make_child(prototype_id);

            let property_symbol_id = SymbolId::new(0);
            let prototype_value = Value::Integer(1);
            let value = Value::Integer(2);

            nia_assert_is_ok(&arena.set_property(
                child_id,
                property_symbol_id,
                value,
            ));
            nia_assert_is_ok(&arena.set_property(
                prototype_id,
                property_symbol_id,
                prototype_value,
            ));

            nia_assert_equal(
                Ok(Some(prototype_value)),
                arena.get_property_value(prototype_id, property_symbol_id),
            );
            nia_assert_equal(
                Ok(Some(value)),
                arena.get_property_value(child_id, property_symbol_id),
            );
        }

        #[test]
        fn deletes_property_from_object() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));

            nia_assert_equal(
                Ok(true),
                arena.has_property(object_id, property_symbol_id),
            );
            nia_assert_is_ok(
                &arena.delete_property(object_id, property_symbol_id),
            );
            nia_assert_equal(
                Ok(false),
                arena.has_property(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_attempts_to_delete_nonexistent_property() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();
            let property_symbol_id = SymbolId::new(0);

            nia_assert_equal(
                Ok(false),
                arena.has_property(object_id, property_symbol_id),
            );
            nia_assert_is_err(
                &arena.delete_property(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_object_was_not_found() {
            let mut arena = ObjectArena::new();
            let object_id = make_nonexistent_object_id();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_err(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));
            nia_assert_is_err(
                &arena.get_property_value(object_id, property_symbol_id),
            );
            nia_assert_is_err(
                &arena.delete_property(object_id, property_symbol_id),
            );
            nia_assert_is_err(
                &arena.has_property(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_adding_new_property_to_frozen_object() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.freeze(object_id));

            nia_assert_is_err(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));
            nia_assert_equal(
                Ok(false),
                arena.has_property(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_changing_property_of_frozen_object() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);
            let new_value = Value::Integer(2);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));
            nia_assert_is_ok(&arena.freeze(object_id));

            nia_assert_equal(
                Ok(Some(value)),
                arena.get_property_value(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property(
                object_id,
                property_symbol_id,
                new_value,
            ));
            nia_assert_equal(
                Ok(Some(value)),
                arena.get_property_value(object_id, property_symbol_id),
            );
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_property_flags__get_property_flags {
        use super::*;
        use crate::OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT;
        use crate::OBJECT_VALUE_WRAPPER_FLAGS_NONE;

        #[test]
        fn sets_and_gets_property_flags() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));

            nia_assert_equal(
                Ok(OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT),
                arena.get_property_flags(object_id, property_symbol_id),
            );
            nia_assert_equal(
                Ok(()),
                arena.set_property_flags(
                    object_id,
                    property_symbol_id,
                    OBJECT_VALUE_WRAPPER_FLAGS_NONE,
                ),
            );
            nia_assert_equal(
                Ok(OBJECT_VALUE_WRAPPER_FLAGS_NONE),
                arena.get_property_flags(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_failure_when_object_does_not_exist() {
            let mut arena = ObjectArena::new();
            let object_id = make_nonexistent_object_id();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_err(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));

            nia_assert_is_err(
                &arena.get_property_flags(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_flags(
                object_id,
                property_symbol_id,
                OBJECT_VALUE_WRAPPER_FLAGS_NONE,
            ));
            nia_assert_is_err(
                &arena.get_property_flags(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_configuring_property_of_frozen_object() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));
            nia_assert_is_ok(&arena.freeze(object_id));

            nia_assert_equal(
                Ok(OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT),
                arena.get_property_flags(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_flags(
                object_id,
                property_symbol_id,
                OBJECT_VALUE_WRAPPER_FLAGS_NONE,
            ));
            nia_assert_equal(
                Ok(OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT),
                arena.get_property_flags(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_object_property_does_not_exist() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_err(
                &arena.get_property_flags(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_flags(
                object_id,
                property_symbol_id,
                OBJECT_VALUE_WRAPPER_FLAGS_NONE,
            ));
            nia_assert_is_err(
                &arena.get_property_flags(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_configuring_not_configurable_property() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));
            nia_assert_is_ok(&arena.set_property_flags(
                object_id,
                property_symbol_id,
                OBJECT_VALUE_WRAPPER_FLAGS_NONE,
            ));

            nia_assert_equal(
                Ok(OBJECT_VALUE_WRAPPER_FLAGS_NONE),
                arena.get_property_flags(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_flags(
                object_id,
                property_symbol_id,
                OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT,
            ));
            nia_assert_equal(
                Ok(OBJECT_VALUE_WRAPPER_FLAGS_NONE),
                arena.get_property_flags(object_id, property_symbol_id),
            );
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_property_flag__get_property_flag {
        use super::*;
        use crate::OBJECT_VALUE_WRAPPER_FLAGS_NONE;
        use crate::OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE;

        #[test]
        fn sets_and_gets_property_flags() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));

            nia_assert_equal(
                Ok(true),
                arena.get_property_flag(
                    object_id,
                    property_symbol_id,
                    OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE,
                ),
            );
            nia_assert_equal(
                Ok(()),
                arena.set_property_flag(
                    object_id,
                    property_symbol_id,
                    OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE,
                    false,
                ),
            );
            nia_assert_equal(
                Ok(false),
                arena.get_property_flag(
                    object_id,
                    property_symbol_id,
                    OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE,
                ),
            );
        }

        #[test]
        fn returns_failure_when_object_does_not_exist() {
            let mut arena = ObjectArena::new();
            let object_id = make_nonexistent_object_id();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_err(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));

            nia_assert_is_err(
                &arena.get_property_flags(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_flags(
                object_id,
                property_symbol_id,
                OBJECT_VALUE_WRAPPER_FLAGS_NONE,
            ));
            nia_assert_is_err(
                &arena.get_property_flags(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_configuring_property_of_frozen_object() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));
            nia_assert_is_ok(&arena.freeze(object_id));

            nia_assert_equal(
                Ok(true),
                arena.get_property_flag(
                    object_id,
                    property_symbol_id,
                    OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE,
                ),
            );
            nia_assert_is_err(&arena.set_property_flags(
                object_id,
                property_symbol_id,
                OBJECT_VALUE_WRAPPER_FLAGS_NONE,
            ));
            nia_assert_equal(
                Ok(true),
                arena.get_property_flag(
                    object_id,
                    property_symbol_id,
                    OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE,
                ),
            );
        }

        #[test]
        fn returns_error_when_object_property_does_not_exist() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_err(
                &arena.get_property_flags(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_flags(
                object_id,
                property_symbol_id,
                OBJECT_VALUE_WRAPPER_FLAGS_NONE,
            ));
            nia_assert_is_err(
                &arena.get_property_flags(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_configuring_not_configurable_property() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_err(
                &arena.get_property_flags(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_flags(
                object_id,
                property_symbol_id,
                OBJECT_VALUE_WRAPPER_FLAGS_NONE,
            ));
            nia_assert_is_err(
                &arena.get_property_flags(object_id, property_symbol_id),
            );
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_property_internable__is_property_internable {
        use super::*;

        use crate::OBJECT_VALUE_WRAPPER_FLAGS_NONE;
        use crate::OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE;

        #[test]
        fn sets_and_gets_internable_flag() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));

            nia_assert_equal(
                Ok(true),
                arena.is_property_internable(object_id, property_symbol_id),
            );
            nia_assert_is_ok(&arena.set_property_internable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_equal(
                Ok(false),
                arena.is_property_internable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_failure_when_object_does_not_exist() {
            let mut arena = ObjectArena::new();
            let object_id = make_nonexistent_object_id();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_err(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));

            nia_assert_is_err(
                &arena.is_property_internable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_internable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_is_err(
                &arena.is_property_internable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_configuring_property_of_frozen_object() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));
            nia_assert_is_ok(&arena.freeze(object_id));

            nia_assert_equal(
                Ok(true),
                arena.is_property_internable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_internable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_equal(
                Ok(true),
                arena.is_property_internable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_object_property_does_not_exist() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);

            nia_assert_is_err(
                &arena.is_property_internable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_internable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_is_err(
                &arena.is_property_internable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_configuring_not_configurable_property() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));
            nia_assert_is_ok(&arena.set_property_configurable(
                object_id,
                property_symbol_id,
                false,
            ));

            nia_assert_equal(
                Ok(true),
                arena.is_property_internable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_internable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_equal(
                Ok(true),
                arena.is_property_internable(object_id, property_symbol_id),
            );
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_property_writable__is_property_writable {
        use super::*;

        use crate::OBJECT_VALUE_WRAPPER_FLAGS_NONE;
        use crate::OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE;

        #[test]
        fn sets_and_gets_writable_flag() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));

            nia_assert_equal(
                Ok(true),
                arena.is_property_writable(object_id, property_symbol_id),
            );
            nia_assert_is_ok(&arena.set_property_writable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_equal(
                Ok(false),
                arena.is_property_writable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_failure_when_object_does_not_exist() {
            let mut arena = ObjectArena::new();
            let object_id = make_nonexistent_object_id();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_err(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));

            nia_assert_is_err(
                &arena.is_property_writable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_writable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_is_err(
                &arena.is_property_writable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_configuring_property_of_frozen_object() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));
            nia_assert_is_ok(&arena.freeze(object_id));

            nia_assert_equal(
                Ok(true),
                arena.is_property_writable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_writable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_equal(
                Ok(true),
                arena.is_property_writable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_object_property_does_not_exist() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_err(
                &arena.is_property_writable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_writable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_is_err(
                &arena.is_property_writable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_configuring_not_configurable_property() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));
            nia_assert_is_ok(&arena.set_property_configurable(
                object_id,
                property_symbol_id,
                false,
            ));

            nia_assert_equal(
                Ok(true),
                arena.is_property_writable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_writable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_equal(
                Ok(true),
                arena.is_property_writable(object_id, property_symbol_id),
            );
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_property_enumerable__is_property_enumerable {
        use super::*;

        use crate::OBJECT_VALUE_WRAPPER_FLAGS_NONE;
        use crate::OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE;

        #[test]
        fn sets_and_gets_enumerable_flag() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));

            nia_assert_equal(
                Ok(true),
                arena.is_property_enumerable(object_id, property_symbol_id),
            );
            nia_assert_is_ok(&arena.set_property_enumerable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_equal(
                Ok(false),
                arena.is_property_enumerable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_failure_when_object_does_not_exist() {
            let mut arena = ObjectArena::new();
            let object_id = make_nonexistent_object_id();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_err(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));

            nia_assert_is_err(
                &arena.is_property_enumerable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_enumerable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_is_err(
                &arena.is_property_enumerable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_configuring_property_of_frozen_object() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));
            nia_assert_is_ok(&arena.freeze(object_id));

            nia_assert_equal(
                Ok(true),
                arena.is_property_enumerable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_enumerable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_equal(
                Ok(true),
                arena.is_property_enumerable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_object_property_does_not_exist() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_err(
                &arena.is_property_enumerable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_enumerable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_is_err(
                &arena.is_property_enumerable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_configuring_not_configurable_property() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));
            nia_assert_is_ok(&arena.set_property_configurable(
                object_id,
                property_symbol_id,
                false,
            ));

            nia_assert_equal(
                Ok(true),
                arena.is_property_enumerable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_enumerable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_equal(
                Ok(true),
                arena.is_property_enumerable(object_id, property_symbol_id),
            );
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_property_configurable__is_property_configurable {
        use super::*;

        use crate::OBJECT_VALUE_WRAPPER_FLAGS_NONE;
        use crate::OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE;

        #[test]
        fn sets_and_gets_configurable_flag() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));

            nia_assert_equal(
                Ok(true),
                arena.is_property_configurable(object_id, property_symbol_id),
            );
            nia_assert_is_ok(&arena.set_property_configurable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_equal(
                Ok(false),
                arena.is_property_configurable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_failure_when_object_does_not_exist() {
            let mut arena = ObjectArena::new();
            let object_id = make_nonexistent_object_id();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_err(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));

            nia_assert_is_err(
                &arena.is_property_configurable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_configurable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_is_err(
                &arena.is_property_configurable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_configuring_property_of_frozen_object() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));
            nia_assert_is_ok(&arena.freeze(object_id));

            nia_assert_equal(
                Ok(true),
                arena.is_property_configurable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_configurable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_equal(
                Ok(true),
                arena.is_property_configurable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_object_property_does_not_exist() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_err(
                &arena.is_property_configurable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_configurable(
                object_id,
                property_symbol_id,
                false,
            ));
            nia_assert_is_err(
                &arena.is_property_configurable(object_id, property_symbol_id),
            );
        }

        #[test]
        fn returns_error_when_configuring_not_configurable_property() {
            let mut arena = ObjectArena::new();
            let object_id = arena.make();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_is_ok(&arena.set_property(
                object_id,
                property_symbol_id,
                value,
            ));
            nia_assert_is_ok(&arena.set_property_configurable(
                object_id,
                property_symbol_id,
                false,
            ));

            nia_assert_equal(
                Ok(false),
                arena.is_property_configurable(object_id, property_symbol_id),
            );
            nia_assert_is_err(&arena.set_property_configurable(
                object_id,
                property_symbol_id,
                true,
            ));
            nia_assert_equal(
                Ok(false),
                arena.is_property_configurable(object_id, property_symbol_id),
            );
        }
    }
}
