use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::library;

pub fn is_root_alist_has_key(
    interpreter: &mut Interpreter,
    key: Value,
    name: &str,
) -> Result<bool, Error> {
    let root_alist = library::get_root_variable(interpreter, name)?;

    let result =
        library::alist_contains_key_question(interpreter, key, root_alist)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    const EMPTY_ALIST_VARIABLE_SYMBOL_NAME: &'static str = "test-alist-symbol";

    fn setup(interpreter: &mut Interpreter, alist: Value) {
        let root_environment_id = interpreter.get_root_environment_id();
        let symbol =
            interpreter.intern_symbol_id(EMPTY_ALIST_VARIABLE_SYMBOL_NAME);

        interpreter
            .define_variable(root_environment_id, symbol, alist)
            .unwrap();
    }

    #[test]
    fn returns_true_if_root_alist_has_key() {
        let mut interpreter = Interpreter::new();

        let alist = nia_alist!(
            interpreter,
            (Value::Integer(1), Value::Integer(11)),
            (Value::Integer(2), Value::Integer(12)),
            (Value::Integer(3), Value::Integer(13))
        );

        setup(&mut interpreter, alist);

        let specs = vec![
            (false, Value::Integer(0)),
            (true, Value::Integer(1)),
            (true, Value::Integer(2)),
            (true, Value::Integer(3)),
            (false, Value::Integer(4)),
        ];

        for (expected, key) in specs {
            let result = is_root_alist_has_key(
                &mut interpreter,
                key,
                EMPTY_ALIST_VARIABLE_SYMBOL_NAME,
            )
            .unwrap();

            nia_assert_equal(expected, result)
        }
    }
}
