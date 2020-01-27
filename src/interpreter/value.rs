use crate::interpreter::cons::Cons;

#[derive(Debug, Clone)]
pub struct Function {
    // environment: LexicalEnvironment
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Keyword(String),
    Symbol(String),
    String(String),
    Cons(Cons),
    Function(Function)
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;

        match (self, other) {
            (Integer(val1), Integer(val2)) => val1 == val2,
            (Float(val1), Float(val2)) => val1 == val2,
            (Boolean(val1), Boolean(val2)) => val1 == val2,
            (Keyword(val1), Keyword(val2)) => val1 == val2,
            (Symbol(val1), Symbol(val2)) => val1 == val2,
            (String(val1), String(val2)) => val1 == val2,
            (Cons(val1), Cons(val2)) => val1 == val2,
//            (Function(val1), Function(val2)) => val1 == val2,
            _ => false
        }
    }

}

