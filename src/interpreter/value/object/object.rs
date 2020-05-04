use std::collections::HashMap;

use crate::interpreter::value::ObjectId;
use crate::interpreter::value::ObjectValueWrapper;
use crate::interpreter::value::SymbolId;
use crate::interpreter::value::Value;

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object {
    properties: HashMap<SymbolId, ObjectValueWrapper>,
    prototype: Option<ObjectId>,
    frozen: bool,
}

impl Object {
    pub fn new() -> Object {
        Object {
            properties: HashMap::new(),
            prototype: None,
            frozen: false,
        }
    }

    pub fn new_child(object_id: ObjectId) -> Object {
        Object {
            properties: HashMap::new(),
            prototype: Some(object_id),
            frozen: false,
        }
    }

    pub fn get_prototype(&self) -> Option<ObjectId> {
        self.prototype
    }

    pub fn set_prototype(&mut self, prototype_id: ObjectId) -> Result<(), Error> {
        self.check_is_not_frozen()?;

        self.prototype = Some(prototype_id);

        Ok(())
    }

    pub fn clear_prototype(&mut self) -> Result<(), Error> {
        self.check_is_not_frozen()?;

        self.prototype = None;

        Ok(())
    }

    pub fn is_frozen(&self) -> bool {
        self.frozen
    }

    pub fn freeze(&mut self) -> Result<(), Error> {
        self.check_is_not_frozen()?;

        self.frozen = true;

        Ok(())
    }

    pub fn check_is_not_frozen(&self) -> Result<(), Error> {
        if self.frozen {
            Error::generic_execution_error("Cannot change frozen object.").into()
        } else {
            Ok(())
        }
    }

    fn get_property_value_wrapper(
        &self,
        property_symbol_id: SymbolId,
    ) -> Option<&ObjectValueWrapper> {
        match self.properties.get(&property_symbol_id) {
            Some(object_value_wrapper) => Some(object_value_wrapper),
            None => None,
        }
    }

    fn get_property_value_wrapper_mut(
        &mut self,
        property_symbol_id: SymbolId,
    ) -> Option<&mut ObjectValueWrapper> {
        match self.properties.get_mut(&property_symbol_id) {
            Some(object_value_wrapper) => Some(object_value_wrapper),
            None => None,
        }
    }

    fn get_property_value_wrapper_required(
        &self,
        property_symbol_id: SymbolId,
    ) -> Result<&ObjectValueWrapper, Error> {
        let object_property_wrapper = self
            .get_property_value_wrapper(property_symbol_id)
            .ok_or_else(|| Error::generic_execution_error("Cannot find object property."))?;

        Ok(object_property_wrapper)
    }

    fn get_property_value_wrapper_mut_required(
        &mut self,
        property_symbol_id: SymbolId,
    ) -> Result<&mut ObjectValueWrapper, Error> {
        let object_property_wrapper = self
            .get_property_value_wrapper_mut(property_symbol_id)
            .ok_or_else(|| Error::generic_execution_error("Cannot find object property."))?;

        Ok(object_property_wrapper)
    }

    pub fn has_property(&self, symbol_id: SymbolId) -> bool {
        self.properties.contains_key(&symbol_id)
    }

    pub fn get_property_value(&self, property_symbol_id: SymbolId) -> Result<Option<Value>, Error> {
        match self.get_property_value_wrapper(property_symbol_id) {
            Some(object_value_wrapper) => Ok(Some(object_value_wrapper.get_value()?)),
            None => Ok(None),
        }
    }

    pub fn set_property(
        &mut self,
        property_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.check_is_not_frozen()?;

        match self.get_property_value_wrapper_mut(property_symbol_id) {
            Some(object_value_wrapper) => {
                object_value_wrapper.set_value(value)?;
            }
            None => {
                self.properties
                    .insert(property_symbol_id, ObjectValueWrapper::new(value));
            }
        }

        Ok(())
    }

    pub fn delete_property(&mut self, property_symbol_id: SymbolId) -> Result<(), Error> {
        self.check_is_not_frozen()?;

        match self.properties.remove(&property_symbol_id) {
            Some(_) => Ok(()),
            None => Error::generic_execution_error("Object has no property to delete.").into(),
        }
    }

    pub fn get_property_flags(&self, property_symbol_id: SymbolId) -> Result<u8, Error> {
        match self.get_property_value_wrapper(property_symbol_id) {
            Some(object_value_wrapper) => Ok(object_value_wrapper.get_flags()),
            None => Error::generic_execution_error("Cannot find object property.").into(),
        }
    }

    pub fn set_property_flags(
        &mut self,
        property_symbol_id: SymbolId,
        flags: u8,
    ) -> Result<(), Error> {
        self.check_is_not_frozen()?;

        let object_property_wrapper = self
            .get_property_value_wrapper_mut(property_symbol_id)
            .ok_or_else(|| Error::generic_execution_error("Cannot find object property."))?;

        object_property_wrapper.set_flags(flags)
    }

    pub fn get_property_flag(&self, property_symbol_id: SymbolId, flag: u8) -> Result<bool, Error> {
        let flag = self
            .get_property_value_wrapper_required(property_symbol_id)?
            .get_flag(flag);

        Ok(flag)
    }

    pub fn set_property_flag(
        &mut self,
        property_symbol_id: SymbolId,
        flag: u8,
        flag_value: bool,
    ) -> Result<(), Error> {
        self.check_is_not_frozen()?;

        self.get_property_value_wrapper_mut_required(property_symbol_id)?
            .set_flag(flag, flag_value)
    }

    pub fn is_property_internable(&self, property_symbol_id: SymbolId) -> Result<bool, Error> {
        let object_property_wrapper =
            self.get_property_value_wrapper_required(property_symbol_id)?;

        Ok(object_property_wrapper.is_internable())
    }

    pub fn is_property_writable(&self, property_symbol_id: SymbolId) -> Result<bool, Error> {
        let object_property_wrapper =
            self.get_property_value_wrapper_required(property_symbol_id)?;

        Ok(object_property_wrapper.is_writable())
    }

    pub fn is_property_enumerable(&self, property_symbol_id: SymbolId) -> Result<bool, Error> {
        let object_property_wrapper =
            self.get_property_value_wrapper_required(property_symbol_id)?;

        Ok(object_property_wrapper.is_enumerable())
    }

    pub fn is_property_configurable(&self, property_symbol_id: SymbolId) -> Result<bool, Error> {
        let object_property_wrapper =
            self.get_property_value_wrapper_required(property_symbol_id)?;

        Ok(object_property_wrapper.is_configurable())
    }

    pub fn set_property_internable(
        &mut self,
        property_symbol_id: SymbolId,
        flag_value: bool,
    ) -> Result<(), Error> {
        self.check_is_not_frozen()?;

        let object_property_wrapper =
            self.get_property_value_wrapper_mut_required(property_symbol_id)?;

        object_property_wrapper.set_internable(flag_value)
    }

    pub fn set_property_writable(
        &mut self,
        property_symbol_id: SymbolId,
        flag_value: bool,
    ) -> Result<(), Error> {
        self.check_is_not_frozen()?;

        let object_property_wrapper =
            self.get_property_value_wrapper_mut_required(property_symbol_id)?;

        object_property_wrapper.set_writable(flag_value)
    }

    pub fn set_property_enumerable(
        &mut self,
        property_symbol_id: SymbolId,
        flag_value: bool,
    ) -> Result<(), Error> {
        self.check_is_not_frozen()?;

        let object_property_wrapper =
            self.get_property_value_wrapper_mut_required(property_symbol_id)?;

        object_property_wrapper.set_enumerable(flag_value)
    }

    pub fn set_property_configurable(
        &mut self,
        property_symbol_id: SymbolId,
        flag_value: bool,
    ) -> Result<(), Error> {
        self.check_is_not_frozen()?;

        let object_property_wrapper =
            self.get_property_value_wrapper_mut_required(property_symbol_id)?;

        object_property_wrapper.set_configurable(flag_value)
    }

    pub fn get_properties(&self) -> &HashMap<SymbolId, ObjectValueWrapper> {
        &self.properties
    }

    pub fn get_gc_items(&self) -> Vec<Value> {
        let mut result = self
            .properties
            .keys()
            .into_iter()
            .map(|symbol_id| Value::Symbol(*symbol_id))
            .collect::<Vec<Value>>();

        result.extend(
            self.properties
                .values()
                .into_iter()
                .map(|value_wrapper| value_wrapper.force_get_value()),
        );

        match self.prototype {
            Some(prototype_id) => result.push(Value::Object(prototype_id)),
            _ => {}
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_prototype__clear_prototype__get_prototype {
        use super::*;

        #[allow(non_snake_case)]
        #[test]
        fn sets_prototype__clears_prototype__gets_prototype() {
            let parent_object_id = ObjectId::new(0);
            let mut object = Object::new();

            nia_assert_equal(None, object.get_prototype());
            nia_assert_equal(Ok(()), object.set_prototype(parent_object_id));
            nia_assert_equal(Some(parent_object_id), object.get_prototype());
            nia_assert_equal(Ok(()), object.clear_prototype());
            nia_assert_equal(None, object.get_prototype());
        }

        #[test]
        fn when_object_is_frozen_changing_prototype_returns_error() {
            let parent_object_id = ObjectId::new(0);
            let new_parent_object_id = ObjectId::new(1);
            let mut object = Object::new();

            nia_assert_is_ok(&object.set_prototype(parent_object_id));
            nia_assert_equal(Ok(()), object.freeze());

            nia_assert_equal(Some(parent_object_id), object.get_prototype());

            nia_assert_is_err(&object.set_prototype(new_parent_object_id));
            nia_assert_equal(Some(parent_object_id), object.get_prototype());

            nia_assert_is_err(&object.clear_prototype());
            nia_assert_equal(Some(parent_object_id), object.get_prototype())
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod has_property__delete_property__set_property__get_property {
        use super::*;

        #[test]
        fn gets_and_sets_properties() {
            let mut object = Object::new();

            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            nia_assert_equal(false, object.has_property(property_symbol_id));
            nia_assert_equal(Ok(None), object.get_property_value(property_symbol_id));

            nia_assert_is_ok(&object.set_property(property_symbol_id, value));
            nia_assert_equal(true, object.has_property(property_symbol_id));
            nia_assert_equal(
                Ok(Some(Value::Integer(1))),
                object.get_property_value(property_symbol_id),
            );
        }

        #[test]
        fn able_to_set_value_again() {
            let mut object = Object::new();

            let property_symbol_id = SymbolId::new(0);
            let value1 = Value::Integer(1);
            let value2 = Value::Integer(2);

            nia_assert_equal(false, object.has_property(property_symbol_id));
            nia_assert_equal(Ok(None), object.get_property_value(property_symbol_id));

            nia_assert_is_ok(&object.set_property(property_symbol_id, value1));
            nia_assert_equal(true, object.has_property(property_symbol_id));
            nia_assert_equal(
                Ok(Some(Value::Integer(1))),
                object.get_property_value(property_symbol_id),
            );

            nia_assert_is_ok(&object.set_property(property_symbol_id, value2));
            nia_assert_equal(true, object.has_property(property_symbol_id));
            nia_assert_equal(
                Ok(Some(Value::Integer(2))),
                object.get_property_value(property_symbol_id),
            );
        }

        #[test]
        fn returns_none_if_no_value_were_set() {
            let object = Object::new();

            let property_symbol_id = SymbolId::new(0);

            nia_assert_equal(false, object.has_property(property_symbol_id));
            nia_assert_equal(Ok(None), object.get_property_value(property_symbol_id));
        }

        #[test]
        fn deletes_existing_property() {
            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(0);

            let mut object = Object::new();

            nia_assert_is_ok(&object.set_property(property_symbol_id, value));

            nia_assert_is_ok(&object.delete_property(property_symbol_id));
            nia_assert_equal(false, object.has_property(property_symbol_id));
        }

        #[test]
        fn when_deleting_nonexistent_property_returns_error() {
            let property_symbol_id = SymbolId::new(0);
            let mut object = Object::new();

            nia_assert_is_err(&object.delete_property(property_symbol_id));
        }

        #[test]
        fn when_object_is_frozen_adding_new_items_returns_errors() {
            let property_symbol_id = SymbolId::new(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();
            nia_assert_equal(Ok(()), object.freeze());

            nia_assert_equal(Ok(None), object.get_property_value(property_symbol_id));
            nia_assert(object.set_property(property_symbol_id, new_value).is_err());
            nia_assert_equal(Ok(None), object.get_property_value(property_symbol_id));
        }

        #[test]
        fn when_object_is_frozen_changing_existing_items_returns_error() {
            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();

            nia_assert_is_ok(&object.set_property(property_symbol_id, value));
            nia_assert_equal(Ok(()), object.freeze());

            nia_assert_equal(
                Ok(Some(value)),
                object.get_property_value(property_symbol_id),
            );
            nia_assert_is_err(&object.set_property(property_symbol_id, new_value));
            nia_assert_equal(
                Ok(Some(value)),
                object.get_property_value(property_symbol_id),
            );
        }

        #[test]
        fn when_object_is_frozen_deleting_existing_property_returns_error() {
            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(0);

            let mut object = Object::new();

            nia_assert_is_ok(&object.set_property(property_symbol_id, value));
            nia_assert_equal(Ok(()), object.freeze());

            nia_assert_is_err(&object.delete_property(property_symbol_id));
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod is_frozen {
        use super::*;

        #[test]
        fn returns_false_when_object_is_not_frozen() {
            let object = Object::new();

            nia_assert_equal(false, object.is_frozen());
        }

        #[test]
        fn returns_true_when_object_is_frozen() {
            let mut object = Object::new();

            nia_assert_equal(false, object.is_frozen());
            nia_assert_is_ok(&object.freeze());
            nia_assert_equal(true, object.is_frozen());
        }
    }

    #[cfg(test)]
    mod freeze {
        use super::*;
        use crate::interpreter::value::value::Value::Symbol;

        #[test]
        fn freezes_object() {
            let mut object = Object::new();

            nia_assert_equal(false, object.is_frozen());
            nia_assert_is_ok(&object.freeze());
            nia_assert_equal(true, object.is_frozen());
        }

        #[test]
        fn freezing_frozen_object_returns_error() {
            let mut object = Object::new();

            nia_assert_is_ok(&object.freeze());
            nia_assert_is_err(&object.freeze());
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_property_flags__get_property_flags {
        use super::*;
        use crate::{
            OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT, OBJECT_VALUE_WRAPPER_FLAGS_NONE,
            OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE, OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE,
        };

        #[test]
        fn gets_and_sets_property_flags() {
            let property_symbol_id = SymbolId::new(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();
            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, new_value));

            nia_assert_equal(
                Ok(OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT),
                object.get_property_flags(property_symbol_id),
            );
            nia_assert(
                object
                    .set_property_flags(property_symbol_id, OBJECT_VALUE_WRAPPER_FLAGS_NONE)
                    .is_ok(),
            );
            nia_assert_equal(
                Ok(OBJECT_VALUE_WRAPPER_FLAGS_NONE),
                object.get_property_flags(property_symbol_id),
            );
        }

        #[test]
        fn when_object_is_frozen_configuring_property_returns_errors() {
            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            let mut object = Object::new();

            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, value));
            nia_assert_equal(Ok(()), object.freeze());

            nia_assert_is_err(
                &object.set_property_flags(property_symbol_id, OBJECT_VALUE_WRAPPER_FLAGS_NONE),
            );
        }

        #[test]
        fn returns_error_when_used_for_non_existing_properties() {
            let property_symbol_id = SymbolId::new(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();

            nia_assert_is_err(&object.get_property_flags(property_symbol_id));
            nia_assert_is_err(
                &object.set_property_flags(property_symbol_id, OBJECT_VALUE_WRAPPER_FLAGS_NONE),
            );
            nia_assert_is_err(&object.get_property_flags(property_symbol_id));
        }

        #[test]
        fn returns_error_when_configuring_not_configurable_property() {
            let property_symbol_id = SymbolId::new(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();
            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, new_value));

            nia_assert_is_ok(
                &object.set_property_flags(property_symbol_id, OBJECT_VALUE_WRAPPER_FLAGS_NONE),
            );

            nia_assert_equal(
                Ok(OBJECT_VALUE_WRAPPER_FLAGS_NONE),
                object.get_property_flags(property_symbol_id),
            );
            nia_assert_is_err(
                &object.set_property_flags(property_symbol_id, OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT),
            );
            nia_assert_equal(
                Ok(OBJECT_VALUE_WRAPPER_FLAGS_NONE),
                object.get_property_flags(property_symbol_id),
            );
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_property_flag__get_property_flag {
        use super::*;
        use crate::{
            OBJECT_VALUE_WRAPPER_FLAGS_NONE, OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE,
            OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE,
        };

        #[test]
        fn gets_and_sets_property_flag() {
            let property_symbol_id = SymbolId::new(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();
            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, new_value));

            nia_assert_equal(
                Ok(()),
                object
                    .set_property_flags(property_symbol_id, OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE),
            );
            nia_assert_equal(
                Ok(false),
                object.get_property_flag(property_symbol_id, OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE),
            );
            nia_assert_equal(
                Ok(()),
                object.set_property_flag(
                    property_symbol_id,
                    OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE,
                    true,
                ),
            );
            nia_assert_equal(
                Ok(true),
                object.get_property_flag(property_symbol_id, OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE),
            );
        }

        #[test]
        fn when_object_is_frozen_configuring_property_returns_errors() {
            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            let mut object = Object::new();

            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, value));
            nia_assert_equal(Ok(()), object.freeze());

            nia_assert_is_err(&object.set_property_flag(
                property_symbol_id,
                OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE,
                false,
            ));
        }

        #[test]
        fn returns_error_when_used_with_non_existing_properties() {
            let property_symbol_id = SymbolId::new(0);

            let mut object = Object::new();

            nia_assert_is_err(&object.set_property_flag(
                property_symbol_id,
                OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE,
                false,
            ));
            nia_assert_is_err(&object.set_property_flag(
                property_symbol_id,
                OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE,
                false,
            ));
            nia_assert_is_err(
                &object.get_property_flag(property_symbol_id, OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE),
            );
        }

        #[test]
        fn returns_error_when_configuring_not_configurable_property() {
            let property_symbol_id = SymbolId::new(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();
            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, new_value));

            nia_assert_is_ok(&object.set_property_flag(
                property_symbol_id,
                OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE,
                false,
            ));

            nia_assert_equal(Ok(true), object.is_property_internable(property_symbol_id));
            nia_assert_is_err(&object.set_property_flag(
                property_symbol_id,
                OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE,
                false,
            ));
            nia_assert_equal(Ok(true), object.is_property_internable(property_symbol_id));
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_property_internable__is_property_internable {
        use super::*;

        #[test]
        fn gets_and_sets_internable_flag() {
            let property_symbol_id = SymbolId::new(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();

            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, new_value));

            nia_assert_equal(Ok(true), object.is_property_internable(property_symbol_id));
            nia_assert_equal(
                Ok(()),
                object.set_property_internable(property_symbol_id, false),
            );
            nia_assert_equal(Ok(false), object.is_property_internable(property_symbol_id));
        }

        #[test]
        fn when_object_is_frozen_setting_internable_flag_returns_error() {
            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            let mut object = Object::new();

            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, value));
            nia_assert_equal(Ok(()), object.freeze());

            nia_assert_is_err(&object.set_property_internable(property_symbol_id, false));
        }

        #[test]
        fn returns_error_when_used_with_non_existing_property() {
            let property_symbol_id = SymbolId::new(0);
            let mut object = Object::new();

            nia_assert_is_err(&object.is_property_internable(property_symbol_id));
            nia_assert_is_err(&object.set_property_internable(property_symbol_id, false));
            nia_assert_is_err(&object.is_property_internable(property_symbol_id));
        }

        #[test]
        fn returns_error_when_configuring_not_configurable_property() {
            let property_symbol_id = SymbolId::new(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();
            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, new_value));

            nia_assert_equal(
                Ok(()),
                object.set_property_configurable(property_symbol_id, false),
            );
            nia_assert_is_err(&object.set_property_internable(property_symbol_id, false));
            nia_assert_equal(
                Ok((true)),
                object.is_property_internable(property_symbol_id),
            );
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_property_writable__is_property_writable {
        use super::*;

        #[test]
        fn gets_and_sets_writable_flag() {
            let property_symbol_id = SymbolId::new(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();

            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, new_value));

            nia_assert_equal(Ok(true), object.is_property_writable(property_symbol_id));
            nia_assert_equal(
                Ok(()),
                object.set_property_writable(property_symbol_id, false),
            );
            nia_assert_equal(Ok(false), object.is_property_writable(property_symbol_id));
        }

        #[test]
        fn when_object_is_frozen_setting_writable_flag_returns_error() {
            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            let mut object = Object::new();

            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, value));
            nia_assert_equal(Ok(()), object.freeze());

            nia_assert_is_err(&object.set_property_writable(property_symbol_id, false));
        }

        #[test]
        fn returns_error_when_used_with_non_existing_property() {
            let property_symbol_id = SymbolId::new(0);
            let mut object = Object::new();

            nia_assert_is_err(&object.is_property_writable(property_symbol_id));
            nia_assert_is_err(&object.set_property_writable(property_symbol_id, false));
            nia_assert_is_err(&object.is_property_writable(property_symbol_id));
        }

        #[test]
        fn returns_error_when_configuring_not_configurable_property() {
            let property_symbol_id = SymbolId::new(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();
            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, new_value));

            nia_assert_equal(
                Ok(()),
                object.set_property_configurable(property_symbol_id, false),
            );
            nia_assert_is_err(&object.set_property_writable(property_symbol_id, false));
            nia_assert_equal(Ok((true)), object.is_property_writable(property_symbol_id));
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_property_enumerable__is_property_enumerable {
        use super::*;

        #[test]
        fn gets_and_sets_enumerable_flag() {
            let property_symbol_id = SymbolId::new(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();

            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, new_value));

            nia_assert_equal(Ok(true), object.is_property_enumerable(property_symbol_id));
            nia_assert_equal(
                Ok(()),
                object.set_property_enumerable(property_symbol_id, false),
            );
            nia_assert_equal(Ok(false), object.is_property_enumerable(property_symbol_id));
        }

        #[test]
        fn when_object_is_frozen_setting_enumerable_flag_returns_error() {
            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            let mut object = Object::new();

            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, value));
            nia_assert_equal(Ok(()), object.freeze());

            nia_assert_is_err(&object.set_property_enumerable(property_symbol_id, false));
        }

        #[test]
        fn returns_error_when_used_with_non_existing_property() {
            let property_symbol_id = SymbolId::new(0);
            let mut object = Object::new();

            nia_assert_is_err(&object.is_property_enumerable(property_symbol_id));
            nia_assert_is_err(&object.set_property_enumerable(property_symbol_id, false));
            nia_assert_is_err(&object.is_property_enumerable(property_symbol_id));
        }

        #[test]
        fn returns_error_when_configuring_not_configurable_property() {
            let property_symbol_id = SymbolId::new(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();
            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, new_value));

            nia_assert_equal(
                Ok(()),
                object.set_property_configurable(property_symbol_id, false),
            );
            nia_assert_is_err(&object.set_property_enumerable(property_symbol_id, false));
            nia_assert_equal(
                Ok((true)),
                object.is_property_enumerable(property_symbol_id),
            );
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_property_configurable__is_property_configurable {
        use super::*;

        #[test]
        fn gets_and_sets_configurable_flag() {
            let property_symbol_id = SymbolId::new(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();

            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, new_value));

            nia_assert_equal(
                Ok(true),
                object.is_property_configurable(property_symbol_id),
            );
            nia_assert_equal(
                Ok(()),
                object.set_property_configurable(property_symbol_id, false),
            );
            nia_assert_equal(
                Ok(false),
                object.is_property_configurable(property_symbol_id),
            );
        }

        #[test]
        fn when_object_is_frozen_setting_configurable_flag_returns_error() {
            let property_symbol_id = SymbolId::new(0);
            let value = Value::Integer(1);

            let mut object = Object::new();

            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, value));
            nia_assert_equal(Ok(()), object.freeze());

            nia_assert_is_err(&object.set_property_configurable(property_symbol_id, false));
        }

        #[test]
        fn returns_error_when_used_with_non_existing_property() {
            let property_symbol_id = SymbolId::new(0);
            let mut object = Object::new();

            nia_assert_is_err(&object.is_property_configurable(property_symbol_id));
            nia_assert_is_err(&object.set_property_configurable(property_symbol_id, false));
            nia_assert_is_err(&object.is_property_configurable(property_symbol_id));
        }

        #[test]
        fn returns_error_when_configuring_not_configurable_property() {
            let property_symbol_id = SymbolId::new(0);
            let new_value = Value::Integer(1);

            let mut object = Object::new();
            nia_assert_equal(Ok(()), object.set_property(property_symbol_id, new_value));

            nia_assert_equal(
                Ok(()),
                object.set_property_configurable(property_symbol_id, false),
            );
            nia_assert_is_err(&object.set_property_configurable(property_symbol_id, true));
            nia_assert_equal(
                Ok(false),
                object.is_property_configurable(property_symbol_id),
            );
        }
    }
}
