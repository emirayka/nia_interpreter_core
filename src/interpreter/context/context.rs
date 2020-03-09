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
}

#[cfg(test)]
mod tests {
    use super::*;


}