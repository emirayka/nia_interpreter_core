use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

#[derive(Debug, Clone)]
pub struct Cons {
    car: Box<Value>,
    cdr: Box<Value>
}

impl Cons {
    pub fn new(car: Value, cdr: Value) -> Cons {
        Cons {
            car: Box::new(car),
            cdr: Box::new(cdr)
        }
    }
}

impl Cons {
    pub fn get_car(&self) -> &Value {
        self.car.as_ref()
    }

    pub fn get_cdr(&self) -> &Value {
        self.cdr.as_ref()
    }

    pub fn get_car_mut(&mut self) -> &mut Value {
        self.car.as_mut()
    }

    pub fn get_cdr_mut(&mut self) -> &mut Value {
        self.cdr.as_mut()
    }

    pub fn set_car(&mut self, new_car: Value) {
        self.car = Box::new(new_car);
    }

    pub fn set_cdr(&mut self, new_cdr: Value) {
        self.cdr = Box::new(new_cdr);
    }
}

impl Cons {
    pub fn to_vec(&self) -> Vec<Value> {
        let mut vector = Vec::new();
        let mut current_cdr = self;

        loop {
            vector.push(current_cdr.get_car().clone());

            current_cdr = match current_cdr.get_cdr() {
                Value::Cons(cons) => cons,
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
}

impl PartialEq for Cons {
    fn eq(&self, other: &Self) -> bool {
        self.car == other.car &&
            self.cdr == other.cdr
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

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_car__set_cdr {
        use super::*;

        #[test]
        fn test_works_correctly() {
            let mut l = Cons::new(Value::String("car".to_string()), Value::String("cdr".to_string()));

            assert_eq!(&Value::String("car".to_string()), l.get_car());
            assert_eq!(&Value::String("cdr".to_string()), l.get_cdr());

            l.set_car(Value::Integer(1));
            l.set_cdr(Value::Integer(2));

            assert_eq!(&Value::Integer(1), l.get_car());
            assert_eq!(&Value::Integer(2), l.get_cdr());
        }
    }


    #[cfg(test)]
    mod to_vec {
        use super::*;

        #[test]
        fn test_returns_correct_vector_that_represents_values_in_cons_cells() {
            let cons = Cons::new(
                Value::Integer(1),
                Value::Cons(Cons::new(
                    Value::Integer(2),
                    Value::Cons(Cons::new(
                        Value::Integer(3),
                        Value::Symbol(new_symbol("nil"))
                    ))
                ))
            );

            let result_vector = cons.to_vec();

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
            let construct_cons= |v: &Value| Cons::new(
                Value::Integer(1),
                Value::Cons(Cons::new(
                    Value::Integer(2),
                    Value::Cons(Cons::new(
                        Value::Integer(3),
                        v.clone()
                    ))
                ))
            );

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
                let incorrect_cons = construct_cons(&incorrect_cdr);

                let result = incorrect_cons.to_vec();

                assert_eq!(&incorrect_cdr, result.last().unwrap());
            }
        }
    }
}
