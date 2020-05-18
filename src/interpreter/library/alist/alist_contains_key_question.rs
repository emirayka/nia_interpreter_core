use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::Value;

use crate::library;

pub fn alist_contains_key_question(
    interpreter: &mut Interpreter,
    key: Value,
    alist: Value,
) -> Result<bool, Error> {
    let alist_vector = library::read_as_vector(interpreter, alist)?;

    for alist_key_value_pair in alist_vector {
        let alist_key_value_cons_id =
            library::read_as_cons_id(alist_key_value_pair)?;

        let alist_key = interpreter.get_car(alist_key_value_cons_id)?;

        if library::deep_equal(interpreter, alist_key, key)? {
            return Ok(true);
        }
    }

    Ok(false)
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
    fn returns_true_if_key_value_pair_exists() {
        let mut interpreter = Interpreter::new();

        let alist = nia_alist!(
            interpreter,
            (Value::Integer(1), Value::Boolean(false)),
            (Value::Integer(2), Value::Boolean(false)),
            (Value::Integer(3), Value::Boolean(false))
        );

        let specs = vec![
            (false, Value::Integer(0)),
            (true, Value::Integer(1)),
            (true, Value::Integer(2)),
            (true, Value::Integer(3)),
            (false, Value::Integer(4)),
        ];

        for (expected, key) in specs {
            let result =
                alist_contains_key_question(&mut interpreter, key, alist)
                    .unwrap();

            nia_assert_equal(expected, result)
        }
    }

    #[test]
    fn returns_invalid_argument_error_when_not_an_alist_was_passed() {
        let mut interpreter = Interpreter::new();

        let alist = nia_alist!(interpreter);

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
            let result =
                alist_contains_key_question(&mut interpreter, spec, spec);

            crate::utils::assert_invalid_argument_error(&result);
        }
    }
}
