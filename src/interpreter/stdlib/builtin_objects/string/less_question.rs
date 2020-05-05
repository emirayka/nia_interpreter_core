use std::cmp::Ordering;

use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;

pub fn less_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `string:less?' takes two arguments exactly",
        )
        .into();
    }

    let mut values = values;

    let string1 = library::read_as_string(interpreter, values.remove(0))?;
    let string2 = library::read_as_string(interpreter, values.remove(0))?;

    let result = match string1.cmp(string2) {
        Ordering::Less => Value::Boolean(true),
        Ordering::Equal => Value::Boolean(false),
        Ordering::Greater => Value::Boolean(false),
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_correct_comparison_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (r#"(string:less? "" "a")"#, Value::Boolean(true)),
            (r#"(string:less? "a" "")"#, Value::Boolean(false)),
            (r#"(string:less? "abc" "abc")"#, Value::Boolean(false)),
            (r#"(string:less? "abc" "def")"#, Value::Boolean(true)),
            (r#"(string:less? "def" "def")"#, Value::Boolean(false)),
            (r#"(string:less? "def" "abc")"#, Value::Boolean(false)),
            (r#"(string:less? "abc" "ab")"#, Value::Boolean(false)),
            (r#"(string:less? "abc" "de")"#, Value::Boolean(true)),
            (r#"(string:less? "ab" "abc")"#, Value::Boolean(true)),
            (r#"(string:less? "de" "abc")"#, Value::Boolean(false)),
            (r#"(string:less? "" "")"#, Value::Boolean(false)),
            (r#"(string:less? "a" "a")"#, Value::Boolean(false)),
            (r#"(string:less? "ab" "ab")"#, Value::Boolean(false)),
            (r#"(string:less? "abc" "abc")"#, Value::Boolean(false)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_was_called_with_invalid_argument_count(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:less?)"#,
            r#"(string:less? "a")"#,
            r#"(string:less? "a" "b" "c")"#,
        ];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_when_was_called_with_not_strings() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:less? "a" 1)"#,
            r#"(string:less? "a" 1.1)"#,
            r#"(string:less? "a" #t)"#,
            r#"(string:less? "a" #f)"#,
            r#"(string:less? "a" 'symbol)"#,
            r#"(string:less? "a" :keyword)"#,
            r#"(string:less? "a" {:object-key 'value})"#,
            r#"(string:less? "a" (cons 1 2))"#,
            r#"(string:less? "a" #(+ %1 %2))"#,
            r#"(string:less? 1 "b")"#,
            r#"(string:less? 1.1 "b")"#,
            r#"(string:less? #t "b")"#,
            r#"(string:less? #f "b")"#,
            r#"(string:less? 'symbol "b")"#,
            r#"(string:less? :keyword "b")"#,
            r#"(string:less? {:object-key 'value} "b")"#,
            r#"(string:less? (cons 1 2) "b")"#,
            r#"(string:less? #(+ %1 %2) "b")"#,
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
