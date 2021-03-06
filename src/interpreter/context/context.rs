use crate::interpreter::error::Error;
use crate::interpreter::value::SymbolId;
use crate::interpreter::value::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Context {
    values: HashMap<SymbolId, Value>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            values: HashMap::new(),
        }
    }

    pub fn get_value(&self, symbol_id: SymbolId) -> Result<Value, Error> {
        match self.values.get(&symbol_id) {
            Some(value) => Ok(*value),
            _ => Error::failure(format!(
                "Cannot find context value with id: {}",
                symbol_id.get_id()
            ))
            .into(),
        }
    }

    pub fn set_value(
        &mut self,
        symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        match self.values.get_mut(&symbol_id) {
            Some(mut_value_ref) => {
                *mut_value_ref = value;
            },
            _ => {
                self.values.insert(symbol_id, value);
            },
        };

        Ok(())
    }

    pub fn get_gc_items(&self) -> Vec<Value> {
        let mut result: Vec<Value> = self
            .values
            .keys()
            .into_iter()
            .map(|symbol_id| symbol_id.into())
            .collect();

        result.extend(self.values.values().into_iter());

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod get_value__set_value {
        use super::*;

        #[test]
        fn returns_found_value() {
            let mut context = Context::new();

            let expected = Value::Integer(1);

            context.set_value(SymbolId::new(0), expected).unwrap();
            let result = context.get_value(SymbolId::new(0));

            nia_assert_equal(expected, result.unwrap());
        }

        #[test]
        fn returns_err_when_no_value_was_found() {
            let context = Context::new();

            let result = context.get_value(SymbolId::new(0));

            nia_assert_is_err(&result);
        }
    }
}
