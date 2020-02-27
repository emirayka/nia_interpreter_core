use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

pub fn object_make(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() % 2 != 0 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `object:make' must take even count of arguments."
        ).into_result();
    }

    let mut values = values;
    let object_id = interpreter.make_object();

    while values.len() > 0 {
        let key = values.remove(0);
        let value = values.remove(0);

        if let Value::Keyword(keyword_id) = key {
            let keyword = interpreter.get_keyword(keyword_id)
                .map_err(|err| interpreter.make_generic_execution_error_caused(
                    "",
                    err
                ))?;

            let keyword_name = keyword.get_name().clone(); // todo: fix, looks ugly

            let symbol_id = interpreter.intern(&keyword_name);

            interpreter.set_object_item(
                object_id,
                symbol_id,
                value
            ).map_err(|err| interpreter.make_generic_execution_error_caused(
                "",
                err
            ))?;
        } else {
            return interpreter.make_invalid_argument_error(
                "Every even argument of built-in function `object:make' must be a keyword."
            ).into_result();
        }
    }

    Ok(Value::Object(object_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;
    use crate::interpreter::string::string_arena::StringId;

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
                let symbol_id = interpreter.intern(key);

                assert_eq!(value, interpreter.get_object_item(object_id, symbol_id).unwrap().unwrap());
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
                ("b", Value::String(StringId::new(0))),
            ),
            "(object:make :a 1 :b \"string\")"
         );
    }

    #[test]
    fn returns_invalid_argument_error_when_odd_count_of_arguments_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(object:make :a)",
            "(object:make :a 1 :b)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_when_an_even_argument_is_not_a_keyword() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(object:make 1 1)",
            "(object:make 1.1 1)",
            "(object:make #t 1)",
            "(object:make #f 1)",
            "(object:make 'symbol 1)",
            "(object:make \"string\" 1)",
            "(object:make {} 1)",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}