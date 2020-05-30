use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::Error;
use crate::Key;

pub fn list_to_key(
    interpreter: &mut Interpreter,
    key_list: Value,
) -> Result<Key, Error> {
    let key = match key_list {
        Value::Integer(key_code) => nia_key!(key_code as i32),
        Value::Cons(cons_id) => {
            let mut key_values = interpreter.list_to_vec(cons_id)?;

            if key_values.len() != 2 {
                return Error::invalid_argument_error(
                    "List must have two items exactly to be parsed as a key."
                ).into()
            }

            match (key_values.remove(0), key_values.remove(0)) {
                (Value::Integer(device_id), Value::Integer(key_code)) => {
                    nia_key!(device_id as i32, key_code as i32)
                },
                _ => return Error::invalid_argument_error(
                    "List must consist of two integer items to be parsed as a key."
                ).into()
            }
        },
        _ => return Error::invalid_argument_error(
            "Value must be an integer or a list of two integers to be parsed as a key."
        ).into()
    };

    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn parses_correctly_lists() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (nia_key!(1), "1"),
            (nia_key!(2), "2"),
            (nia_key!(1, 1), "'(1 1)"),
            (nia_key!(1, 2), "'(1 2)"),
            (nia_key!(2, 1), "'(2 1)"),
            (nia_key!(2, 2), "'(2 2)"),
        ];

        for (expected, value) in specs {
            let result =
                interpreter.execute_in_main_environment(value).unwrap();
            let result = list_to_key(&mut interpreter, result).unwrap();

            nia_assert_equal(expected.get_device_id(), result.get_device_id());

            nia_assert_equal(expected.get_key_id(), result.get_key_id());
        }
    }

    #[test]
    fn returns_invalid_argument_errors_when_invalid_argument_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "1.1",
            "#t",
            "#f",
            "\"string\"",
            ":keyword",
            "'symbol",
            "'()",
            "'(1)",
            "'(1 2 3)",
            "{}",
            "#()",
        ];

        for spec in specs {
            let result = interpreter.execute_in_main_environment(spec).unwrap();
            let result = list_to_key(&mut interpreter, result);

            utils::assert_invalid_argument_error(&result);
        }
    }
}
