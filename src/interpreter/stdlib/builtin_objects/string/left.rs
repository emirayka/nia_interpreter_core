use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;
use std::cmp;

pub fn left(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `string:left' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let count = library::read_as_i64(values.remove(0))?;

    let string = library::read_as_string(interpreter, values.remove(0))?;

    let len = string.chars().count() as i64;

    let (left_index, right_index) = if count >= 0 {
        (0usize, cmp::min(count, len) as usize)
    } else {
        (cmp::max(len + count, 0) as usize, count as usize)
    };

    let mut chars = string.chars();

    for _ in 0..left_index {
        chars.next();
    }

    let result = chars.take(right_index - left_index).collect::<String>();

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
    fn returns_left_symbols() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (r#"(string:left -5 "abcd")"#, r#""abcd""#),
            (r#"(string:left -4 "abcd")"#, r#""abcd""#),
            (r#"(string:left -3 "abcd")"#, r#""bcd""#),
            (r#"(string:left -2 "abcd")"#, r#""cd""#),
            (r#"(string:left -1 "abcd")"#, r#""d""#),
            (r#"(string:left 0 "abcd")"#, r#""""#),
            (r#"(string:left 1 "abcd")"#, r#""a""#),
            (r#"(string:left 2 "abcd")"#, r#""ab""#),
            (r#"(string:left 3 "abcd")"#, r#""abc""#),
            (r#"(string:left 4 "abcd")"#, r#""abcd""#),
            (r#"(string:left 5 "abcd")"#, r#""abcd""#),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn handles_unicode() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (r#"(string:left -5 "猫a钥b匙c月")"#, r#""钥b匙c月""#),
            (r#"(string:left -4 "猫a钥b匙c月")"#, r#""b匙c月""#),
            (r#"(string:left -3 "猫a钥b匙c月")"#, r#""匙c月""#),
            (r#"(string:left -2 "猫a钥b匙c月")"#, r#""c月""#),
            (r#"(string:left -1 "猫a钥b匙c月")"#, r#""月""#),
            (r#"(string:left 0 "猫a钥b匙c月")"#, r#""""#),
            (r#"(string:left 1 "猫a钥b匙c月")"#, r#""猫""#),
            (r#"(string:left 2 "猫a钥b匙c月")"#, r#""猫a""#),
            (r#"(string:left 3 "猫a钥b匙c月")"#, r#""猫a钥""#),
            (r#"(string:left 4 "猫a钥b匙c月")"#, r#""猫a钥b""#),
            (r#"(string:left 5 "猫a钥b匙c月")"#, r#""猫a钥b匙""#),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_was_called_with_invalid_count_of_arguments(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:left)"#,
            r#"(string:left 3)"#,
            r#"(string:left 3 "b" "c")"#,
        ];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_was_called_with_invalid_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:left 1.1 "test")"#,
            r#"(string:left #t "test")"#,
            r#"(string:left #f "test")"#,
            r#"(string:left "test" "test")"#,
            r#"(string:left 'symbol "test")"#,
            r#"(string:left :keyword "test")"#,
            r#"(string:left {:object-key 'value} "test")"#,
            r#"(string:left (cons 1 2) "test")"#,
            r#"(string:left #(+ %1 %2) "test")"#,
            r#"(string:left 3 1)"#,
            r#"(string:left 3 1.1)"#,
            r#"(string:left 3 #t)"#,
            r#"(string:left 3 #f)"#,
            r#"(string:left 3 'symbol)"#,
            r#"(string:left 3 :keyword)"#,
            r#"(string:left 3 {:object-key 'value})"#,
            r#"(string:left 3 (cons 1 2))"#,
            r#"(string:left 3 #(+ %1 %2))"#,
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
