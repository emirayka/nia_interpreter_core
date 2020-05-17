use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::SymbolId;
use crate::Value;

use crate::library;

pub fn alist_remove_by_key(
    interpreter: &mut Interpreter,
    key: Value,
    alist: Value,
) -> Result<Value, Error> {
    let alist_vector = library::read_as_vector(interpreter, alist)?;
    let mut new_alist_vector = vec![];

    for alist_key_value_pair in alist_vector {
        let alist_key_value_pair_cons_id =
            library::read_as_cons_id(alist_key_value_pair)?;
        let alist_key =
            interpreter.get_car(alist_key_value_pair_cons_id).unwrap();

        if !library::deep_equal(interpreter, key, alist_key)? {
            new_alist_vector.push(alist_key_value_pair_cons_id.into())
        }
    }

    Ok(interpreter.vec_to_list(new_alist_vector))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;
    use crate::{FunctionId, KeywordId, ObjectId, StringId};

    #[test]
    fn returns_value_associated_with_key_or_none_if_there_are_no_such_key() {
        let mut interpreter = Interpreter::new();

        let mut alist = nia_alist!(
            interpreter,
            (Value::Integer(1), Value::Integer(11)),
            (Value::Integer(2), Value::Integer(12)),
            (Value::Integer(3), Value::Integer(13))
        );

        let specs = vec![
            (
                Value::Integer(0),
                nia_alist!(
                    interpreter,
                    (Value::Integer(1), Value::Integer(11)),
                    (Value::Integer(2), Value::Integer(12)),
                    (Value::Integer(3), Value::Integer(13))
                ),
            ),
            (
                Value::Integer(1),
                nia_alist!(
                    interpreter,
                    (Value::Integer(2), Value::Integer(12)),
                    (Value::Integer(3), Value::Integer(13))
                ),
            ),
            (
                Value::Integer(2),
                nia_alist!(
                    interpreter,
                    (Value::Integer(3), Value::Integer(13))
                ),
            ),
            (Value::Integer(3), nia_alist!(interpreter)),
            (Value::Integer(4), nia_alist!(interpreter)),
        ];

        for (key, expected) in specs {
            alist = alist_remove_by_key(&mut interpreter, key, alist).unwrap();

            utils::assert_deep_equal(&mut interpreter, expected, alist);
        }
    }

    #[test]
    fn returns_invalid_argument_error_when_not_an_alist_was_passed() {
        let mut interpreter = Interpreter::new();

        let alist = library::alist_new(&mut interpreter).unwrap();

        let specs = vec![
            Value::Integer(1),
            Value::Float(1.1),
            Value::Boolean(false),
            Value::Boolean(true),
            interpreter.intern_string_value("string"),
            interpreter.intern_keyword_value("keyword"),
            interpreter.intern_symbol_value("symbol"),
            interpreter.make_object_value(),
            interpreter.execute_in_main_environment("#()").unwrap(),
        ];

        for spec in specs {
            let result = alist_remove_by_key(&mut interpreter, spec, spec);

            crate::utils::assert_invalid_argument_error(&result);
        }
    }
}
