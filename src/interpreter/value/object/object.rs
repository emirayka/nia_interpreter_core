use std::collections::HashMap;

use crate::interpreter::value::SymbolId;
use crate::interpreter::value::ObjectId;
use crate::interpreter::value::ObjectValueWrapper;
use crate::interpreter::value::Value;

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object {
    items: HashMap<SymbolId, ObjectValueWrapper>,
    prototype: Option<ObjectId>,
    frozen: bool,
}

impl Object {
    pub fn new() -> Object {
        Object {
            items: HashMap::new(),
            prototype: None,
            frozen: false,
        }
    }

    pub fn new_child(object_id: ObjectId) -> Object {
        Object {
            items: HashMap::new(),
            prototype: Some(object_id),
            frozen: false,
        }
    }

    pub fn get_prototype(&self) -> Option<ObjectId> {
        self.prototype
    }

    pub fn set_prototype(&mut self, object_id: ObjectId) -> Result<(), Error> {
        self.check_if_frozen()?;

        self.prototype = Some(object_id);

        Ok(())
    }

    pub fn has_item(&self, symbol_id: SymbolId) -> bool {
        self.items.contains_key(&symbol_id)
    }

    pub fn is_frozen(&self) -> bool {
        self.frozen
    }

    pub fn freeze(&mut self) -> Result<(), Error> {
        self.check_if_frozen()?;

        self.frozen = true;

        Ok(())
    }

    pub fn check_if_frozen(&self) -> Result<(), Error> {
        if self.frozen {
            Error::generic_execution_error("Cannot change frozen object.")
                .into()
        } else {
            Ok(())
        }
    }

    pub fn get_property(&self, symbol_id: SymbolId) -> Result<Option<Value>, Error> {
        match self.items.get(&symbol_id) {
            Some(v) => {
                let value = v.get_value()?;

                Ok(Some(value))
            }
            None => Ok(None)
        }
    }

    pub fn set_property(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        self.check_if_frozen()?;

        match self.items.get_mut(&symbol_id) {
            Some(mut_ref) => {
                mut_ref.set_value(value)?;
            }
            _ => {
                self.items.insert(symbol_id, ObjectValueWrapper::new(value));
            }
        }

        Ok(())
    }

    pub fn configure_property(&mut self, symbol_id: SymbolId, flags: u8) -> Result<(), Error> {
        self.check_if_frozen()?;

        match self.items.get_mut(&symbol_id) {
            Some(mut_ref) => {
                mut_ref.set_flags(flags)?;
            }
            _ => {
                return Error::generic_execution_error("Cannot find object item.")
                    .into();
            }
        }

        Ok(())
    }

    pub fn get_items(&self) -> &HashMap<SymbolId, ObjectValueWrapper> {
        &self.items
    }

    pub fn get_gc_items(&self) -> Vec<Value> {
        let mut result = self.items
            .keys()
            .into_iter()
            .map(|symbol_id| Value::Symbol(*symbol_id))
            .collect::<Vec<Value>>();

        result.extend(
            self.items
                .values()
                .into_iter()
                .map(|value_wrapper| value_wrapper.force_get_value())
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

    #[test]
    fn gets_and_sets_items() {
        let mut object = Object::new();

        let key = SymbolId::new(0);
        let value = Value::Integer(1);

        object.set_property(key, value);

        assert_eq!(Ok(Some(Value::Integer(1))), object.get_property(key));
    }

    #[test]
    fn able_to_set_value_again() {
        let mut object = Object::new();

        let key = SymbolId::new(0);
        let value1 = Value::Integer(1);
        let value2 = Value::Integer(2);

        object.set_property(key, value1);
        object.set_property(key, value2);

        assert_eq!(Ok(Some(Value::Integer(2))), object.get_property(key));
    }

    #[test]
    fn returns_none_if_no_value_was_set() {
        let object = Object::new();

        let key = SymbolId::new(0);

        assert_eq!(Ok(None), object.get_property(key));
    }
}

