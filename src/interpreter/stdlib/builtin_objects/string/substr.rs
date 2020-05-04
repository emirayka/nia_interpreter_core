use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;
use std::cmp;

// todo: maybe change function spec
pub fn substr(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 3 {
        return Error::invalid_argument_count_error(
            "Built-in function `string:substr' takes only one argument.",
        )
        .into();
    }

    let mut values = values;

    let index = library::read_as_i64(values.remove(0))?;

    let diff = library::read_as_i64(values.remove(0))?;

    if index < 0 {
        return Error::invalid_argument_error(
            "The first argument of built-in function `string:substring' must be a positive int value."
        ).into();
    }

    if diff < 0 {
        return Error::invalid_argument_error(
            "The second argument of built-in function `string:substring' must be a positive int value."
        ).into();
    }

    let string = library::read_as_string(interpreter, values.remove(0))?;

    let length = string.chars().count() as i64;

    let first = index;
    let second = index + diff;

    let first = cmp::max(0, first);
    let second = cmp::min(second, length);

    let mut chars = string.chars();

    for _ in 0..first {
        chars.next();
    }

    let result = chars.take((second - first) as usize).collect::<String>();

    Ok(interpreter.intern_string_value(&result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_correct_substring() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (r#"(string:substr 0 0 "abcdef")"#, r#""""#),
            (r#"(string:substr 0 1 "abcdef")"#, r#""a""#),
            (r#"(string:substr 0 2 "abcdef")"#, r#""ab""#),
            (r#"(string:substr 1 0 "abcdef")"#, r#""""#),
            (r#"(string:substr 1 1 "abcdef")"#, r#""b""#),
            (r#"(string:substr 1 2 "abcdef")"#, r#""bc""#),
            (r#"(string:substr 2 0 "abcdef")"#, r#""""#),
            (r#"(string:substr 2 1 "abcdef")"#, r#""c""#),
            (r#"(string:substr 2 2 "abcdef")"#, r#""cd""#),
            (r#"(string:substr 2 0 "abcdef")"#, r#""""#),
            (r#"(string:substr 2 1 "abcdef")"#, r#""c""#),
            (r#"(string:substr 2 2 "abcdef")"#, r#""cd""#),
            (r#"(string:substr 0 10 "abcdef")"#, r#""abcdef""#),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn handles_utf8() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (r#"(string:substr 0 0 "猫a钥b匙c月")"#, r#""""#),
            (r#"(string:substr 0 1 "猫a钥b匙c月")"#, r#""猫""#),
            (r#"(string:substr 0 2 "猫a钥b匙c月")"#, r#""猫a""#),
            (r#"(string:substr 0 3 "猫a钥b匙c月")"#, r#""猫a钥""#),
            (r#"(string:substr 0 4 "猫a钥b匙c月")"#, r#""猫a钥b""#),
            (r#"(string:substr 0 5 "猫a钥b匙c月")"#, r#""猫a钥b匙""#),
            (r#"(string:substr 0 6 "猫a钥b匙c月")"#, r#""猫a钥b匙c""#),
            (r#"(string:substr 0 7 "猫a钥b匙c月")"#, r#""猫a钥b匙c月""#),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }
    #[test]
    fn returns_invalid_argument_count_error_when_was_called_with_invalid_count_of_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:substr)"#,
            r#"(string:substr 1)"#,
            r#"(string:substr 1 2)"#,
            r#"(string:substr 1 2 "test" "")"#,
        ];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_error_when_was_called_with_invalid_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:substr 1.1 2 "test")"#,
            r#"(string:substr #f 2 "test")"#,
            r#"(string:substr #t 2 "test")"#,
            r#"(string:substr "string" 2 "test")"#,
            r#"(string:substr 'symbol 2 "test")"#,
            r#"(string:substr :keyword 2 "test")"#,
            r#"(string:substr '(1 2) 2 "test")"#,
            r#"(string:substr {} 2 "test")"#,
            r#"(string:substr #(+ %1 %2) 2 "test")"#,
            r#"(string:substr 1 1.1 "test")"#,
            r#"(string:substr 1 #f "test")"#,
            r#"(string:substr 1 #t "test")"#,
            r#"(string:substr 1 "string" "test")"#,
            r#"(string:substr 1 'symbol "test")"#,
            r#"(string:substr 1 :keyword "test")"#,
            r#"(string:substr 1 '(1 2) "test")"#,
            r#"(string:substr 1 {} "test")"#,
            r#"(string:substr 1 #(+ %1 %2) "test")"#,
            r#"(string:substr 1 2 1)"#,
            r#"(string:substr 1 2 1.1)"#,
            r#"(string:substr 1 2 #f)"#,
            r#"(string:substr 1 2 #t)"#,
            r#"(string:substr 1 2 'symbol)"#,
            r#"(string:substr 1 2 :keyword)"#,
            r#"(string:substr 1 2 '(1 2))"#,
            r#"(string:substr 1 2 {})"#,
            r#"(string:substr 1 2 #(+ %1 %2))"#,
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }
}
