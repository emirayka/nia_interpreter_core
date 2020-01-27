use crate::interpreter::value::Value;

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

impl PartialEq for Cons {
    fn eq(&self, other: &Self) -> bool {
        self.car == other.car &&
            self.cdr == other.cdr
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::cons::Cons;
    use crate::interpreter::value::Value;

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
