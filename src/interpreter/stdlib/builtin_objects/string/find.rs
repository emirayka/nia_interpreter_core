use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::stdlib::_lib;

pub fn find(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `string:find?' takes two strings exactly."
        ).into_result();
    }

    let mut values = values;

    let string1 = _lib::read_as_string(
        interpreter,
        values.remove(0)
    )?;

    let string2 = _lib::read_as_string(
        interpreter,
        values.remove(0)
    )?;

    match string2.find(string1) {
        Some(byte_index) => {
            let slice = &string2[0..byte_index];
            let character_count = slice.chars().count() as i64;

            Ok(Value::Integer(character_count))
        },
        None => Ok(Value::Integer(-1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_correct_character_index() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:find "" "abc")"#, Value::Integer(0)),

            (r#"(string:find "a" "abc")"#, Value::Integer(0)),
            (r#"(string:find "b" "abc")"#, Value::Integer(1)),
            (r#"(string:find "c" "abc")"#, Value::Integer(2)),
            (r#"(string:find "ab" "abc")"#, Value::Integer(0)),
            (r#"(string:find "ac" "abc")"#, Value::Integer(-1)),
            (r#"(string:find "bc" "abc")"#, Value::Integer(1)),
            (r#"(string:find "abc" "abc")"#, Value::Integer(0)),

            (r#"(string:find "d" "abc")"#, Value::Integer(-1)),
            (r#"(string:find "abcc" "abc")"#, Value::Integer(-1)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn handles_utf8_correctly() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:find "猫a" "猫a钥b匙c月")"#, Value::Integer(0)),
            (r#"(string:find "a钥" "猫a钥b匙c月")"#, Value::Integer(1)),
            (r#"(string:find "钥b" "猫a钥b匙c月")"#, Value::Integer(2)),
            (r#"(string:find "b匙" "猫a钥b匙c月")"#, Value::Integer(3)),
            (r#"(string:find "匙c" "猫a钥b匙c月")"#, Value::Integer(4)),
            (r#"(string:find "c月" "猫a钥b匙c月")"#, Value::Integer(5)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_was_called_with_invalid_argument_count() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            r#"(string:find)"#,
            r#"(string:find "a")"#,
            r#"(string:find "a" "b" "c")"#
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_when_was_called_with_not_strings() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            r#"(string:find "a" 1)"#,
            r#"(string:find "a" 1.1)"#,
            r#"(string:find "a" #t)"#,
            r#"(string:find "a" #f)"#,
            r#"(string:find "a" 'symbol)"#,
            r#"(string:find "a" :keyword)"#,
            r#"(string:find "a" {:object-key 'value})"#,
            r#"(string:find "a" (cons 1 2))"#,
            r#"(string:find "a" #(+ %1 %2))"#,

            r#"(string:find 1 "b")"#,
            r#"(string:find 1.1 "b")"#,
            r#"(string:find #t "b")"#,
            r#"(string:find #f "b")"#,
            r#"(string:find 'symbol "b")"#,
            r#"(string:find :keyword "b")"#,
            r#"(string:find {:object-key 'value} "b")"#,
            r#"(string:find (cons 1 2) "b")"#,
            r#"(string:find #(+ %1 %2) "b")"#,
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}

