use crate::interpreter::symbol::SymbolId;
use crate::interpreter::string::StringId;
use crate::interpreter::keyword::KeywordId;
use crate::interpreter::cons::ConsId;
use crate::interpreter::object::ObjectId;
use crate::interpreter::function::FunctionId;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Keyword(KeywordId),
    Symbol(SymbolId),
    String(StringId),
    Cons(ConsId),
    Object(ObjectId),
    Function(FunctionId),
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
            (Object(val1), Object(val2)) => val1 == val2,
            (Function(val1), Function(val2)) => val1 == val2,
            _ => false
        }
    }
}

impl Eq for Value {}

impl Value {
    pub fn as_string_id(&self) -> StringId {
        match self {
            Value::String(string_id) => *string_id,
            _ => panic!()
        }
    }

    pub fn as_keyword_id(&self) -> KeywordId {
        match self {
            Value::Keyword(keyword_id) => *keyword_id,
            _ => panic!()
        }
    }

    pub fn as_symbol_id(&self) -> SymbolId {
        match self {
            Value::Symbol(symbol_id) => *symbol_id,
            _ => panic!()
        }
    }

    pub fn as_cons_id(&self) -> ConsId {
        match self {
            Value::Cons(cons_id) => *cons_id,
            _ => panic!()
        }
    }

    pub fn as_object_id(&self) -> ObjectId {
        match self {
            Value::Object(object_id) => *object_id,
            _ => panic!()
        }
    }

    pub fn as_function_id(&self) -> FunctionId {
        match self {
            Value::Function(function_id) => *function_id,
            _ => panic!()
        }
    }
}
