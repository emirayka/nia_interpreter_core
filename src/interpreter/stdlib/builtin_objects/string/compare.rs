use std::cmp::Ordering;

use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;

pub fn compare(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `string:compare' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let string1 = library::read_as_string(interpreter, values.remove(0))?;
    let string2 = library::read_as_string(interpreter, values.remove(0))?;

    let result = match string1.cmp(string2) {
        Ordering::Less => Value::Integer(-1),
        Ordering::Equal => Value::Integer(0),
        Ordering::Greater => Value::Integer(1),
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_correct_comparison_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (r#"(string:compare "" "abc")"#, Value::Integer(-1)),
            (r#"(string:compare "abc" "")"#, Value::Integer(1)),
            (r#"(string:compare "abc" "abc")"#, Value::Integer(0)),
            (r#"(string:compare "abc" "def")"#, Value::Integer(-1)),
            (r#"(string:compare "def" "def")"#, Value::Integer(0)),
            (r#"(string:compare "def" "abc")"#, Value::Integer(1)),
            (r#"(string:compare "abc" "ab")"#, Value::Integer(1)),
            (r#"(string:compare "abc" "de")"#, Value::Integer(-1)),
            (r#"(string:compare "ab" "abc")"#, Value::Integer(-1)),
            (r#"(string:compare "de" "abc")"#, Value::Integer(1)),
            (r#"(string:compare "" "")"#, Value::Integer(0)),
            (r#"(string:compare "a" "a")"#, Value::Integer(0)),
            (r#"(string:compare "ab" "ab")"#, Value::Integer(0)),
            (r#"(string:compare "abc" "abc")"#, Value::Integer(0)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_was_called_with_invalid_argument_count(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:compare)"#,
            r#"(string:compare "a")"#,
            r#"(string:compare "a" "b" "c")"#,
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_when_was_called_with_not_strings() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:compare "a" 1)"#,
            r#"(string:compare "a" 1.1)"#,
            r#"(string:compare "a" #t)"#,
            r#"(string:compare "a" #f)"#,
            r#"(string:compare "a" 'symbol)"#,
            r#"(string:compare "a" :keyword)"#,
            r#"(string:compare "a" {:object-key 'value})"#,
            r#"(string:compare "a" (cons:new 1 2))"#,
            r#"(string:compare "a" #(+ %1 %2))"#,
            r#"(string:compare 1 "b")"#,
            r#"(string:compare 1.1 "b")"#,
            r#"(string:compare #t "b")"#,
            r#"(string:compare #f "b")"#,
            r#"(string:compare 'symbol "b")"#,
            r#"(string:compare :keyword "b")"#,
            r#"(string:compare {:object-key 'value} "b")"#,
            r#"(string:compare (cons:new 1 2) "b")"#,
            r#"(string:compare #(+ %1 %2) "b")"#,
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
