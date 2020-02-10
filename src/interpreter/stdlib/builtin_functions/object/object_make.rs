use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn object_make(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() % 2 != 0 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Built-in function `object:make' must take even count of arguments."
        ));
    }

    let mut values = values;
    let object_id = interpreter.make_object();

    while values.len() > 0 {
        let key = values.remove(0);
        let value = values.remove(0);

        if let Value::Keyword(keyword_name) = key {
            let symbol = interpreter.intern_symbol(&keyword_name);

            interpreter.set_object_item(
                object_id,
                &symbol,
                value
            );
        } else {
            return Err(Error::invalid_argument(
                interpreter,
                "Every even argument of built-in function `object:make' must be a keyword."
            ));
        }
    }

    Ok(Value::Object(object_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    macro_rules! assert_object_has_values {
        ($expected:expr, $code:expr) => {
            let expected: Vec<(&str, Value)> = $expected;
            let mut interpreter = Interpreter::new();

            let object_id = if let Value::Object(object_id) = interpreter.execute($code).unwrap() {
                object_id
            } else {
                panic!("");
            };

            for (key, value) in expected {
                let symbol = interpreter.intern_symbol(key);

                assert_eq!(&value, interpreter.get_object_item(object_id, &symbol).unwrap());
            }
        }
    }

    #[test]
    fn makes_new_object() {
        assert_object_has_values!(vec!(), "(object:make)");
    }

    #[test]
    fn correctly_sets_object_values() {
        assert_object_has_values!(
            vec!(
                ("a", Value::Integer(1)),
            ),
            "(object:make :a 1)"
         );

        assert_object_has_values!(
            vec!(
                ("a", Value::Integer(1)),
                ("b", Value::String(String::from("string"))),
            ),
            "(object:make :a 1 :b \"string\")"
         );
    }

    #[test]
    fn returns_invalid_argument_error_when_odd_count_of_arguments_was_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(object:make :a)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(object:make :a 1 :b)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_invalid_argument_when_an_even_argument_is_not_a_keyword() {
        let mut interpreter = Interpreter::new();

        let invalid_arguments = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            "'symbol",
            "\"string\"",
            "{}",
        );

        for invalid_argument in invalid_arguments {
            let result = interpreter.execute(
                &format!("(object:make {} 1)", invalid_argument)
            );
            assertion::assert_invalid_argument_error(&result);
        }
    }
}
