use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::Key;

pub fn key_to_list(interpreter: &mut Interpreter, key: Key) -> Value {
    match key {
        Key::LoneKey(lone_key) => Value::Integer(lone_key.get_key_id() as i64),
        Key::DeviceKey(device_key) => interpreter.vec_to_list(vec![
            Value::Integer(device_key.get_device_id() as i64),
            Value::Integer(device_key.get_key_id() as i64),
        ]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    fn assert_returns_correct_list(s: &str, key: Key) {
        let mut interpreter = Interpreter::new();
        let expected = interpreter.execute_in_main_environment(s).unwrap();
        let result = key_to_list(&mut interpreter, key);

        utils::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn returns_correct_key_chord_part_list_representations() {
        let specs = vec![
            ("0", nia_key!(0)),
            ("'(0 1)", nia_key!(0, 1)),
        ];

        for spec in specs {
            assert_returns_correct_list(spec.0, spec.1);
        }
    }
}
