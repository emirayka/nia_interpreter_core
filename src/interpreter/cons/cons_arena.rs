use std::collections::HashMap;

use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::cons::cons::Cons;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConsId {
    id: usize
}

impl ConsId {
    pub fn new(id: usize) -> ConsId {
        ConsId {
            id
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

pub struct ConsArena {
    arena: HashMap<ConsId, Cons>,
    next_id: usize,
}

impl ConsArena {
    pub fn new() -> ConsArena {
        ConsArena {
            arena: HashMap::new(),
            next_id: 0
        }
    }

    pub fn make_cons(&mut self, car: Value, cdr: Value) -> ConsId {
        let cons = Cons::new(
            car,
            cdr
        );

        let cons_id = ConsId::new(self.next_id);

        self.arena.insert(cons_id, cons);
        self.next_id += 1;

        cons_id
    }
}

impl ConsArena {
    pub fn get_cons(&self, cons_id: &ConsId) -> &Cons {
        self.arena.get(&cons_id).unwrap() // todo: change?
    }

    pub fn get_cons_mut(&mut self, cons_id: &ConsId) -> &mut Cons {
        self.arena.get_mut(&cons_id).unwrap() // todo: change?
    }

    pub fn get_car(&self, cons_id: &ConsId) -> &Value {
        self.get_cons(cons_id).get_car()
    }

    pub fn get_cdr(&self, cons_id: &ConsId) -> &Value {
        self.get_cons(cons_id).get_cdr()
    }

    pub fn get_car_mut(&mut self, cons_id: &ConsId) -> &mut Value {
        self.get_cons_mut(cons_id).get_car_mut()
    }

    pub fn get_cdr_mut(&mut self, cons_id: &ConsId) -> &mut Value {
        self.get_cons_mut(cons_id).get_cdr_mut()
    }

    pub fn set_car(&mut self, cons_id: &ConsId, new_car: Value) {
        self.get_cons_mut(cons_id).set_car(new_car)
    }

    pub fn set_cdr(&mut self, cons_id: &ConsId, new_cdr: Value) {
        self.get_cons_mut(cons_id).set_cdr(new_cdr)
    }
}

impl ConsArena {
    pub fn get_cadr(&self, cons_id: &ConsId) -> Result<Value, Error> {
        let cdr = self.get_cdr(cons_id);

        match cdr {
            Value::Cons(cons_id) => Ok(self.get_car(cons_id).clone()),
            _ => Err(Error::empty())
        }
    }

    pub fn get_cddr(&self, cons_id: &ConsId) -> Result<Value, Error> {
        let cdr = self.get_cdr(cons_id);

        match cdr {
            Value::Cons(cons_id) => Ok(self.get_cdr(cons_id).clone()),
            _ => Err(Error::empty())
        }
    }
}

impl ConsArena {
    pub fn cons_to_vec(&self, cons_id: &ConsId) -> Vec<Value> {
        let mut vector = Vec::new();
        let mut current_cdr = cons_id;

        loop {
            vector.push(self.get_car(current_cdr).clone());

            current_cdr = match self.get_cdr(current_cdr) {
                Value::Cons(cons_id) => &cons_id,
                Value::Symbol(symbol) => {
                    if !symbol.is_nil() {
                        vector.push(Value::Symbol(symbol.clone()));
                    }

                    break;
                }
                value => {
                    vector.push(value.clone());

                    break;
                }
            };
        }

        vector
    }

    pub fn cons_from_vec(&mut self, nil: Value, vector: Vec<Value>) -> Value {
        let mut last_cons = nil;

        for value in vector.into_iter().rev() {
            let cons_id = self.make_cons(value, last_cons);

            last_cons = Value::Cons(cons_id);
        }

        last_cons
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::symbol::{SymbolArena, Symbol};

    fn new_symbol(symbol_name: &str) -> Symbol {
        let mut arena = SymbolArena::new();

        arena.intern(symbol_name)
    }

    fn nil() -> Value {
        Value::Symbol(new_symbol("nil"))
    }

    #[cfg(test)]
    mod cons_to_vec {
        use super::*;

        #[test]
        fn test_returns_correct_vector_that_represents_values_in_cons_cells() {
            let mut cons_arena = ConsArena::new();

            let cdr = Value::Cons(cons_arena.make_cons(
                Value::Integer(3),
                Value::Symbol(new_symbol("nil"))
            ));

            let cdr = Value::Cons(cons_arena.make_cons(
                Value::Integer(2),
                cdr
            ));

            let cons = cons_arena.make_cons(
                Value::Integer(1),
                cdr
            );

            let result_vector = cons_arena.cons_to_vec(&cons);

            assert_eq!(
                vec!(
                    Value::Integer(1),
                    Value::Integer(2),
                    Value::Integer(3)
                ),
                result_vector
            );
        }

        #[test]
        fn test_returns_vector_when_cdr_is_not_nil_nor_cons_cell() {
            let mut cons_arena = ConsArena::new();

            let incorrect_cudders = vec!(
                Value::Integer(1),
                Value::Float(1.1),
                Value::Boolean(true),
                Value::Boolean(false),
                Value::Symbol(new_symbol("not-nil-symbol")),
                Value::String(String::from("string")),
                Value::Keyword(String::from("string")),
                //Value::Function() todo: fix
            );

            for incorrect_cdr in incorrect_cudders {
                let cdr = Value::Cons(cons_arena.make_cons(
                    Value::Integer(3),
                    incorrect_cdr.clone()
                ));

                let cdr = Value::Cons(cons_arena.make_cons(
                    Value::Integer(2),
                    cdr
                ));

                let incorrect_cons = cons_arena.make_cons(
                    Value::Integer(1),
                    cdr
                );

                let result = cons_arena.cons_to_vec(&incorrect_cons);

                assert_eq!(&incorrect_cdr, result.last().unwrap());
            }
        }
    }

    #[cfg(test)]
    mod cons_from_vec {
        use super::*;

        macro_rules! assert_result_eq {
            ($expected:expr, $vector:expr) => {
                let mut cons_arena = ConsArena::new();

                assert_eq!($expected, cons_arena.cons_from_vec(Value::Symbol(new_symbol("nil")), $vector));
            }
        }

        #[test]
        fn returns_nil_for_an_empty_list() {
            assert_result_eq!(nil(), vec!());
        }

        #[test]
        fn returns_list_with_an_value() {
            let values = vec!(
                Value::Integer(1),
                Value::Float(1.1),
                Value::Boolean(true),
                Value::Boolean(false),
            );

            for value in values {
                let mut cons_arena = ConsArena::new();

                assert_result_eq!(
                    Value::Cons(cons_arena.make_cons(value.clone(), nil())),
                    vec!(value)
                );
            }
        }
    }
}
