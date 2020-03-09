use std::collections::HashMap;
use crate::interpreter::symbol::SymbolId;
use crate::interpreter::value::Value;

pub struct Context {
    values: HashMap<SymbolId, Value>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            values: HashMap::new()
        }
    }

    pub fn get_value(&self, symbol_id: SymbolId) -> Result<Value, ()> {
        match self.values.get(&symbol_id) {
            Some(value) => Ok(*value),
            _ => Err(())
        }
    }

    pub fn set_value(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), ()> {
        match self.values.get_mut(&symbol_id) {
            Some(mut_value_ref) => {
                *mut_value_ref = value;
            }
            _ => {
                self.values.insert(symbol_id, value);
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[cfg(test)]
    mod get_value__set_value {
        use super::*;

        #[test]
        fn returns_found_value() {
            let mut context = Context::new();

            let expected = Value::Integer(1);

            context.set_value(SymbolId::new(0), expected);
            let result = context.get_value(SymbolId::new(0));

            assert_eq!(expected, result.unwrap());
        }

        #[test]
        fn returns_err_when_no_value_was_found() {
            let mut context = Context::new();

            let result = context.get_value(SymbolId::new(0));

            assertion::assert_is_error(&result);
        }
    }
}