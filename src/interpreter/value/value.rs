use crate::interpreter::error::Error;
use crate::interpreter::value::{
    ConsId, FunctionId, KeywordId, ObjectId, StringId, SymbolId,
};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(StringId),
    Keyword(KeywordId),
    Symbol(SymbolId),
    Cons(ConsId),
    Object(ObjectId),
    Function(FunctionId),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(v) => write!(f, "Value::Integer({})", v),
            Value::Float(v) => write!(f, "Value::Float({})", v),
            Value::Boolean(v) => write!(f, "Value::Boolean({})", v),
            Value::String(v) => write!(f, "Value::String({})", v),
            Value::Keyword(v) => write!(f, "Value::Keyword({})", v),
            Value::Symbol(v) => write!(f, "Value::Symbol({})", v),
            Value::Cons(v) => write!(f, "Value::Cons({})", v),
            Value::Object(v) => write!(f, "Value::Object({})", v),
            Value::Function(v) => write!(f, "Value::Function({})", v),
        }
    }
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
            _ => false,
        }
    }
}

impl Eq for Value {}

macro_rules! make_value_from_implementation {
    ($from_type_name: ty, $value_variant: path) => {
        impl From<$from_type_name> for Value {
            fn from(v: $from_type_name) -> Self {
                $value_variant(v)
            }
        }

        impl From<&$from_type_name> for Value {
            fn from(v: &$from_type_name) -> Self {
                $value_variant(*v)
            }
        }
    };
}

make_value_from_implementation!(i64, Value::Integer);
make_value_from_implementation!(f64, Value::Float);
make_value_from_implementation!(bool, Value::Boolean);
make_value_from_implementation!(ConsId, Value::Cons);
make_value_from_implementation!(FunctionId, Value::Function);
make_value_from_implementation!(KeywordId, Value::Keyword);
make_value_from_implementation!(ObjectId, Value::Object);
make_value_from_implementation!(StringId, Value::String);
make_value_from_implementation!(SymbolId, Value::Symbol);

// macro_rules! make_value_implementation {
//     ($to_type_name: ty, $value_variant: pat) => {
//         impl From<$from_type_name> for Value {
//             fn from(v: $from_type_name) -> Self {
//                 Value::$value_variant(v)
//             }
//         }
//     }
// }

macro_rules! make_try_from_value_implementation {
    ($try_from_type:ty, $value_variant:path, $type_name:expr) => {
        impl std::convert::TryFrom<Value> for $try_from_type {
            type Error = Error;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                match value {
                    $value_variant(v) => Ok(v),
                    v => Error::failure(format!(
                        "Invalid conversion from {} to {}.",
                        v, $type_name
                    ))
                    .into(),
                }
            }
        }
    };
}

make_try_from_value_implementation!(i64, Value::Integer, "Value::Integer");
make_try_from_value_implementation!(f64, Value::Float, "Value::Float");
make_try_from_value_implementation!(bool, Value::Boolean, "Value::Boolean");
make_try_from_value_implementation!(ConsId, Value::Cons, "Value::Cons");
make_try_from_value_implementation!(
    FunctionId,
    Value::Function,
    "Value::Function"
);
make_try_from_value_implementation!(
    KeywordId,
    Value::Keyword,
    "Value::Keyword"
);
make_try_from_value_implementation!(ObjectId, Value::Object, "Value::Object");
make_try_from_value_implementation!(StringId, Value::String, "Value::String");
make_try_from_value_implementation!(SymbolId, Value::Symbol, "Value::Symbol");

macro_rules! make_value_type_predicate {
    ($name:ident, $variant:path) => {
        impl Value {
            pub fn $name(&self) -> bool {
                match self {
                    $variant(_) => true,
                    _ => false,
                }
            }
        }
    };
}

make_value_type_predicate!(is_integer, Value::Integer);
make_value_type_predicate!(is_float, Value::Float);
make_value_type_predicate!(is_boolean, Value::Boolean);
make_value_type_predicate!(is_string, Value::String);
make_value_type_predicate!(is_keyword, Value::Keyword);
make_value_type_predicate!(is_symbol, Value::Symbol);
make_value_type_predicate!(is_cons, Value::Cons);
make_value_type_predicate!(is_object, Value::Object);
make_value_type_predicate!(is_function, Value::Function);
