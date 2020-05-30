use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::Value;

use crate::library;

pub fn alist_contains_value_question(
    interpreter: &mut Interpreter,
    value: Value,
    alist: Value,
) -> Result<bool, Error> {
    let alist_vector = library::read_as_vector(interpreter, alist)?;

    for alist_value_value_pair in alist_vector {
        let alist_value_value_cons_id =
            library::read_as_cons_id(alist_value_value_pair)?;

        let alist_value = interpreter.get_cdr(alist_value_value_cons_id)?;

        if library::deep_equal(interpreter, alist_value, value)? {
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

    #[test]
    fn returns_true_if_() {
        let mut interpreter = Interpreter::new();

        let alist = nia_alist!(
            interpreter,
            (Value::Integer(1), Value::Integer(11)),
            (Value::Integer(2), Value::Integer(12)),
            (Value::Integer(3), Value::Integer(13))
        );

        let specs = vec![
            (false, Value::Integer(10)),
            (true, Value::Integer(11)),
            (true, Value::Integer(12)),
            (true, Value::Integer(13)),
            (false, Value::Integer(14)),
        ];

        for (expected, value) in specs {
            let result =
                alist_contains_value_question(&mut interpreter, value, alist)
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
                alist_contains_value_question(&mut interpreter, spec, spec);

            crate::utils::assert_invalid_argument_error(&result);
        }
    }
}
