use crate::interpreter::value::Value;

#[derive(Debug, Clone)]
pub struct Cons {
    car: Value,
    cdr: Value
}

impl Cons {
    pub fn new(car: Value, cdr: Value) -> Cons {
        Cons {
            car,
            cdr
        }
    }
}

impl Cons {
    pub fn get_car(&self) -> &Value {
        &self.car
    }

    pub fn get_cdr(&self) -> &Value {
        &self.cdr
    }

    pub fn get_car_mut(&mut self) -> &mut Value {
        &mut self.car
    }

    pub fn get_cdr_mut(&mut self) -> &mut Value {
        &mut self.cdr
    }

    pub fn set_car(&mut self, new_car: Value) {
        self.car = new_car;
    }

    pub fn set_cdr(&mut self, new_cdr: Value) {
        self.cdr = new_cdr;
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

    fn nil() -> Value {
        Value::Symbol(new_symbol("nil"))
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
}