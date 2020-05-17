use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::library;

pub fn remove_item_from_root_alist(
    interpreter: &mut Interpreter,
    name: &str,
    key: Value,
) -> Result<(), Error> {
    let root_alist_value = library::get_root_variable(interpreter, name)?;

    let new_alist_value =
        library::alist_remove_by_key(interpreter, key, root_alist_value)?;

    library::set_root_variable(interpreter, name, new_alist_value)?;

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

    fn setup(interpreter: &mut Interpreter, alist: Value) {
        let root_environment_id = interpreter.get_root_environment_id();
        let symbol =
            interpreter.intern_symbol_id(EMPTY_ALIST_VARIABLE_SYMBOL_NAME);

        interpreter
            .define_variable(root_environment_id, symbol, alist)
            .unwrap();
    }

    #[test]
    fn adds_values_to_root_alist() {
        let mut interpreter = Interpreter::new();

        let alist = nia_alist!(
            interpreter,
            (Value::Integer(1), Value::Integer(11)),
            (Value::Integer(2), Value::Integer(12)),
            (Value::Integer(3), Value::Integer(13))
        );

        setup(&mut interpreter, alist);

        let specs = vec![
            (
                Value::Integer(2),
                nia_alist!(
                    interpreter,
                    (Value::Integer(1), Value::Integer(11)),
                    (Value::Integer(3), Value::Integer(13))
                ),
            ),
            (
                Value::Integer(1),
                nia_alist!(
                    interpreter,
                    (Value::Integer(3), Value::Integer(13))
                ),
            ),
            (Value::Integer(3), nia_alist!(interpreter)),
        ];

        for (key, expected) in specs {
            nia_assert_is_ok(&remove_item_from_root_alist(
                &mut interpreter,
                EMPTY_ALIST_VARIABLE_SYMBOL_NAME,
                key,
            ));

            let result = library::get_root_variable(
                &mut interpreter,
                EMPTY_ALIST_VARIABLE_SYMBOL_NAME,
            )
            .unwrap();

            library::print_value(&mut interpreter, expected);
            library::print_value(&mut interpreter, result);

            utils::assert_deep_equal(&mut interpreter, expected, result)
        }
    }
}
