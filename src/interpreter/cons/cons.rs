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
    pub fn get_car(&self) -> Value {
        self.car
    }

    pub fn get_cdr(&self) -> Value {
        self.cdr
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

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_car__set_cdr {
        use super::*;
        use crate::interpreter::interpreter::Interpreter;
        use crate::interpreter::library::assertion;

        #[test]
        fn works_correctly() {
            let mut interpreter = Interpreter::new();

            let string1 = interpreter.intern_string_value(String::from("car"));
            let string2 = interpreter.intern_string_value(String::from("cdr"));

            let mut l = Cons::new(string1, string2);

            assertion::assert_vectors_deep_equal(
                &mut interpreter,
                vec!(
                    string1,
                    string2,
                ),
                vec!(
                    l.get_car(),
                    l.get_cdr()
                )
            );

            l.set_car(Value::Integer(1));
            l.set_cdr(Value::Integer(2));

            assert_eq!(Value::Integer(1), l.get_car());
            assert_eq!(Value::Integer(2), l.get_cdr());
        }
    }
}
