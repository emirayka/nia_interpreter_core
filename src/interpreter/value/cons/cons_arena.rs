use std::collections::HashMap;

use crate::interpreter::error::Error;
use crate::interpreter::value::{Cons, ConsId, Value};

#[derive(Clone)]
pub struct ConsArena {
    arena: HashMap<ConsId, Cons>,
    next_id: usize,
}

impl ConsArena {
    pub fn new() -> ConsArena {
        ConsArena {
            arena: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn make_cons(&mut self, car: Value, cdr: Value) -> ConsId {
        let cons = Cons::new(car, cdr);

        let cons_id = ConsId::new(self.next_id);

        self.arena.insert(cons_id, cons);
        self.next_id += 1;

        cons_id
    }

    pub fn free_cons(&mut self, cons_id: ConsId) -> Result<(), Error> {
        match self.arena.remove(&cons_id) {
            Some(_) => Ok(()),
            _ => Error::failure(format!(
                "Cannot find a cons with id: {}",
                cons_id.get_id()
            ))
            .into(),
        }
    }

    pub fn get_all_cons_identifiers(&self) -> Vec<ConsId> {
        let mut result = Vec::new();

        for k in self.arena.keys() {
            result.push(*k)
        }

        result
    }
}

impl ConsArena {
    pub fn get_cons(&self, cons_id: ConsId) -> Result<&Cons, Error> {
        match self.arena.get(&cons_id) {
            Some(value) => Ok(value),
            None => Error::failure(format!(
                "Cannot find a cons with id: {}",
                cons_id.get_id()
            ))
            .into(),
        }
    }

    pub fn get_cons_mut(
        &mut self,
        cons_id: ConsId,
    ) -> Result<&mut Cons, Error> {
        match self.arena.get_mut(&cons_id) {
            Some(value) => Ok(value),
            None => Error::failure(format!(
                "Cannot find a cons with id: {}",
                cons_id.get_id()
            ))
            .into(),
        }
    }

    pub fn get_car(&self, cons_id: ConsId) -> Result<Value, Error> {
        match self.get_cons(cons_id) {
            Ok(cons) => Ok(cons.get_car()),
            _ => Error::failure(format!(
                "Cannot find a cons with id: {}",
                cons_id.get_id()
            ))
            .into(),
        }
    }

    pub fn get_cdr(&self, cons_id: ConsId) -> Result<Value, Error> {
        match self.get_cons(cons_id) {
            Ok(cons) => Ok(cons.get_cdr()),
            _ => Error::failure(format!(
                "Cannot find a cons with id: {}",
                cons_id.get_id()
            ))
            .into(),
        }
    }

    pub fn get_car_mut(
        &mut self,
        cons_id: ConsId,
    ) -> Result<&mut Value, Error> {
        match self.get_cons_mut(cons_id) {
            Ok(cons) => Ok(cons.get_car_mut()),
            _ => Error::failure(format!(
                "Cannot find a cons with id: {}",
                cons_id.get_id()
            ))
            .into(),
        }
    }

    pub fn get_cdr_mut(
        &mut self,
        cons_id: ConsId,
    ) -> Result<&mut Value, Error> {
        match self.get_cons_mut(cons_id) {
            Ok(cons) => Ok(cons.get_cdr_mut()),
            _ => Error::failure(format!(
                "Cannot find a cons with id: {}",
                cons_id.get_id()
            ))
            .into(),
        }
    }

    pub fn set_car(
        &mut self,
        cons_id: ConsId,
        new_car: Value,
    ) -> Result<(), Error> {
        match self.get_cons_mut(cons_id) {
            Ok(cons) => {
                cons.set_car(new_car);

                Ok(())
            }
            _ => Error::failure(format!(
                "Cannot find a cons with id: {}",
                cons_id.get_id()
            ))
            .into(),
        }
    }

    pub fn set_cdr(
        &mut self,
        cons_id: ConsId,
        new_cdr: Value,
    ) -> Result<(), Error> {
        match self.get_cons_mut(cons_id) {
            Ok(cons) => {
                cons.set_cdr(new_cdr);

                Ok(())
            }
            _ => Error::failure(format!(
                "Cannot find a cons with id: {}",
                cons_id.get_id()
            ))
            .into(),
        }
    }
}

impl ConsArena {
    pub fn get_cadr(&self, cons_id: ConsId) -> Result<Value, Error> {
        let cdr = self.get_cdr(cons_id)?;

        match cdr {
            Value::Cons(cons_id) => self.get_car(cons_id),
            _ => Error::generic_execution_error(
                "Cannot get car of not a cons cell.",
            )
            .into(),
        }
    }

    pub fn get_cddr(&self, cons_id: ConsId) -> Result<Value, Error> {
        let cdr = self.get_cdr(cons_id)?;

        match cdr {
            Value::Cons(cons_id) => self.get_cdr(cons_id),
            _ => Error::generic_execution_error(
                "Cannot get cdr of not a cons cell.",
            )
            .into(),
        }
    }
}

impl ConsArena {
    pub fn list_to_vec(&self, cons_id: ConsId) -> Result<Vec<Value>, Error> {
        let mut results = Vec::new();
        let mut current_cdr = cons_id;

        loop {
            let value = self.get_car(current_cdr)?;
            results.push(value);

            current_cdr = match self.get_cdr(current_cdr)? {
                Value::Cons(cons_id) => cons_id,
                value => {
                    results.push(value);

                    break;
                }
            };
        }

        Ok(results)
    }

    pub fn vec_to_list(&mut self, nil: Value, vector: Vec<Value>) -> Value {
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

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::interpreter::value::SymbolId;

    fn new_symbol(name: &str) -> SymbolId {
        if name == "nil" {
            SymbolId::new(0)
        } else {
            SymbolId::new(1)
        }
    }

    fn nil() -> Value {
        Value::Symbol(new_symbol("nil"))
    }

    #[cfg(test)]
    mod free_cons {
        use super::*;

        #[test]
        fn removes_cons() {
            let mut cons_arena = ConsArena::new();

            let cons_id =
                cons_arena.make_cons(Value::Integer(1), Value::Integer(1));

            nia_assert(cons_arena.get_cons(cons_id).is_ok());
            nia_assert(cons_arena.free_cons(cons_id).is_ok());
            nia_assert(cons_arena.get_cons(cons_id).is_err());
        }

        #[test]
        fn returns_error_when_attempts_to_remove_cons_with_unknown_id() {
            let mut cons_arena = ConsArena::new();

            let cons_id = ConsId::new(342343);

            nia_assert(cons_arena.free_cons(cons_id).is_err());
        }

        #[test]
        fn returns_error_when_attempts_to_remove_cons_twice() {
            let mut cons_arena = ConsArena::new();

            let cons_id =
                cons_arena.make_cons(Value::Integer(1), Value::Integer(1));

            nia_assert(cons_arena.free_cons(cons_id).is_ok());
            nia_assert(cons_arena.free_cons(cons_id).is_err());
        }
    }

    #[cfg(test)]
    mod list_to_vec {
        use super::*;
        use crate::interpreter::value::FunctionId;
        use crate::interpreter::value::KeywordId;
        use crate::interpreter::value::StringId;

        #[test]
        fn returns_correct_vector_that_represents_values_in_cons_cells() {
            let mut cons_arena = ConsArena::new();

            let cdr = Value::Cons(cons_arena.make_cons(
                Value::Integer(3),
                Value::Symbol(new_symbol("nil")),
            ));

            let cdr = Value::Cons(cons_arena.make_cons(Value::Integer(2), cdr));

            let cons = cons_arena.make_cons(Value::Integer(1), cdr);

            let result_vector = cons_arena.list_to_vec(cons).unwrap();

            nia_assert_equal(
                vec![
                    Value::Integer(1),
                    Value::Integer(2),
                    Value::Integer(3),
                    Value::Symbol(SymbolId::new(0)),
                ],
                result_vector,
            );
        }

        #[test]
        fn returns_vector_when_cdr_is_not_nil_nor_cons_cell() {
            let mut cons_arena = ConsArena::new();

            let incorrect_cudders = vec![
                Value::Integer(1),
                Value::Float(1.1),
                Value::Boolean(true),
                Value::Boolean(false),
                Value::Symbol(new_symbol("not-nil-symbol")),
                Value::String(StringId::new(1024)),
                Value::Keyword(KeywordId::new(1025)),
                Value::Function(FunctionId::new(1026)),
            ];

            for incorrect_cdr in incorrect_cudders {
                let cdr = Value::Cons(
                    cons_arena.make_cons(Value::Integer(3), incorrect_cdr),
                );

                let cdr =
                    Value::Cons(cons_arena.make_cons(Value::Integer(2), cdr));

                let incorrect_cons =
                    cons_arena.make_cons(Value::Integer(1), cdr);

                let result = cons_arena.list_to_vec(incorrect_cons).unwrap();

                nia_assert_equal(&incorrect_cdr, result.last().unwrap());
            }
        }
    }

    #[cfg(test)]
    mod vec_to_list {
        use super::*;

        macro_rules! assert_result_eq {
            ($expected:expr, $vector:expr) => {
                let mut cons_arena = ConsArena::new();

                nia_assert_equal(
                    $expected,
                    cons_arena.vec_to_list(nil(), $vector),
                );
            };
        }

        #[test]
        fn returns_nil_for_an_empty_list() {
            assert_result_eq!(nil(), vec!());
        }

        #[test]
        fn returns_list_with_an_value() {
            let values = vec![
                Value::Integer(1),
                Value::Float(1.1),
                Value::Boolean(true),
                Value::Boolean(false),
            ];

            for value in values {
                let mut cons_arena = ConsArena::new();

                assert_result_eq!(
                    Value::Cons(cons_arena.make_cons(value, nil())),
                    vec!(value)
                );
            }
        }
    }
}
