use std::cmp::Ordering;

use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;

pub fn equal_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `string:equal?' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let string1 = library::read_as_string(interpreter, values.remove(0))?;
    let string2 = library::read_as_string(interpreter, values.remove(0))?;

    let result = match string1.cmp(string2) {
        Ordering::Less => Value::Boolean(false),
        Ordering::Equal => Value::Boolean(true),
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
            (r#"(string:equal? "" "")"#, Value::Boolean(true)),
            (r#"(string:equal? "a" "a")"#, Value::Boolean(true)),
            (r#"(string:equal? "ab" "ab")"#, Value::Boolean(true)),
            (r#"(string:equal? "abc" "abc")"#, Value::Boolean(true)),
            (r#"(string:equal? "abc" "dbc")"#, Value::Boolean(false)),
            (r#"(string:equal? "abc" "adc")"#, Value::Boolean(false)),
            (r#"(string:equal? "abc" "acd")"#, Value::Boolean(false)),
            (r#"(string:equal? "abc" "ab")"#, Value::Boolean(false)),
            (r#"(string:equal? "abc" "bc")"#, Value::Boolean(false)),
            (r#"(string:equal? "abc" "ac")"#, Value::Boolean(false)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_was_called_with_invalid_argument_count(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:equal?)"#,
            r#"(string:equal? "a")"#,
            r#"(string:equal? "a" "b" "c")"#,
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
            r#"(string:equal? "a" 1)"#,
            r#"(string:equal? "a" 1.1)"#,
            r#"(string:equal? "a" #t)"#,
            r#"(string:equal? "a" #f)"#,
            r#"(string:equal? "a" 'symbol)"#,
            r#"(string:equal? "a" :keyword)"#,
            r#"(string:equal? "a" {:object-key 'value})"#,
            r#"(string:equal? "a" (cons 1 2))"#,
            r#"(string:equal? "a" #(+ %1 %2))"#,
            r#"(string:equal? 1 "b")"#,
            r#"(string:equal? 1.1 "b")"#,
            r#"(string:equal? #t "b")"#,
            r#"(string:equal? #f "b")"#,
            r#"(string:equal? 'symbol "b")"#,
            r#"(string:equal? :keyword "b")"#,
            r#"(string:equal? {:object-key 'value} "b")"#,
            r#"(string:equal? (cons 1 2) "b")"#,
            r#"(string:equal? #(+ %1 %2) "b")"#,
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
