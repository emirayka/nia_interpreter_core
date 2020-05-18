use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::library;

pub fn add_item_to_root_alist(
    interpreter: &mut Interpreter,
    key: Value,
    value: Value,
    name: &str,
) -> Result<(), Error> {
    let root_alist_value = library::get_root_variable(interpreter, name)?;

    let root_alist_value =
        library::alist_acons(interpreter, key, value, root_alist_value)?;

    library::set_root_variable(interpreter, name, root_alist_value)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    const EMPTY_ALIST_VARIABLE_SYMBOL_NAME: &'static str = "test-alist-symbol";

    fn setup() -> Interpreter {
        let mut interpreter = Interpreter::new();

        let root_environment_id = interpreter.get_root_environment_id();
        let symbol =
            interpreter.intern_symbol_id(EMPTY_ALIST_VARIABLE_SYMBOL_NAME);
        let alist = nia_alist!(interpreter);

        interpreter
            .define_variable(root_environment_id, symbol, alist)
            .unwrap();

        interpreter
    }

    #[test]
    fn adds_values_to_root_alist() {
        let mut interpreter = setup();

        let specs = vec![
            (
                (Value::Integer(1), Value::Integer(11)),
                nia_alist!(
                    interpreter,
                    (Value::Integer(1), Value::Integer(11))
                ),
            ),
            (
                (Value::Integer(2), Value::Integer(12)),
                nia_alist!(
                    interpreter,
                    (Value::Integer(1), Value::Integer(11)),
                    (Value::Integer(2), Value::Integer(12))
                ),
            ),
            (
                (Value::Integer(3), Value::Integer(13)),
                nia_alist!(
                    interpreter,
                    (Value::Integer(1), Value::Integer(11)),
                    (Value::Integer(2), Value::Integer(12)),
                    (Value::Integer(3), Value::Integer(13))
                ),
            ),
        ];

        for ((key, value), expected) in specs {
            nia_assert_is_ok(&add_item_to_root_alist(
                &mut interpreter,
                key,
                value,
                EMPTY_ALIST_VARIABLE_SYMBOL_NAME,
            ));

            let result = library::get_root_variable(
                &mut interpreter,
                EMPTY_ALIST_VARIABLE_SYMBOL_NAME,
            )
            .unwrap();

            utils::assert_deep_equal(&mut interpreter, expected, result)
        }
    }
}
