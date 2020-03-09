use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use std::cmp;
use crate::interpreter::library;

pub fn right(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `string:right' takes two arguments exactly."
        ).into_result();
    }

    let mut values = values;

    let count = library::read_as_i64(
        interpreter,
        values.remove(0)
    )?;

    let string = library::read_as_string(
        interpreter,
        values.remove(0)
    )?;

    let len = string.chars().count() as i64;

    let (left_index, right_index) = if count >= 0 {
        (cmp::max(0, len - count) as usize, len as usize)
    } else {
        (0, cmp::min(len, -count) as usize)
    };

    let mut chars = string.chars();

    for _ in 0..left_index {
        chars.next();
    }

    let result = chars.take(right_index - left_index).collect::<String>();

    Ok(interpreter.intern_string_value(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_right_symbols() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:right -5 "abcd")"#, r#""abcd""#),
            (r#"(string:right -4 "abcd")"#, r#""abcd""#),
            (r#"(string:right -3 "abcd")"#, r#""abc""#),
            (r#"(string:right -2 "abcd")"#, r#""ab""#),
            (r#"(string:right -1 "abcd")"#, r#""a""#),

            (r#"(string:right 0 "abcd")"#, r#""""#),

            (r#"(string:right 1 "abcd")"#, r#""d""#),
            (r#"(string:right 2 "abcd")"#, r#""cd""#),
            (r#"(string:right 3 "abcd")"#, r#""bcd""#),
            (r#"(string:right 4 "abcd")"#, r#""abcd""#),
            (r#"(string:right 5 "abcd")"#, r#""abcd""#),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn handles_unicode() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:right -5 "猫a钥b匙c月")"#, r#""猫a钥b匙""#),
            (r#"(string:right -4 "猫a钥b匙c月")"#, r#""猫a钥b""#),
            (r#"(string:right -3 "猫a钥b匙c月")"#, r#""猫a钥""#),
            (r#"(string:right -2 "猫a钥b匙c月")"#, r#""猫a""#),
            (r#"(string:right -1 "猫a钥b匙c月")"#, r#""猫""#),

            (r#"(string:right 0 "猫a钥b匙c月")"#, r#""""#),

            (r#"(string:right 1 "猫a钥b匙c月")"#, r#""月""#),
            (r#"(string:right 2 "猫a钥b匙c月")"#, r#""c月""#),
            (r#"(string:right 3 "猫a钥b匙c月")"#, r#""匙c月""#),
            (r#"(string:right 4 "猫a钥b匙c月")"#, r#""b匙c月""#),
            (r#"(string:right 5 "猫a钥b匙c月")"#, r#""钥b匙c月""#),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_was_called_with_invalid_count_of_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            r#"(string:right)"#,
            r#"(string:right 3)"#,
            r#"(string:right 3 "b" "c")"#
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_was_called_with_invalid_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            r#"(string:right 1.1 "test")"#,
            r#"(string:right #t "test")"#,
            r#"(string:right #f "test")"#,
            r#"(string:right 'symbol "test")"#,
            r#"(string:right :keyword "test")"#,
            r#"(string:right {:object-key 'value} "test")"#,
            r#"(string:right (cons 1 2) "test")"#,
            r#"(string:right #(+ %1 %2) "test")"#,

            r#"(string:right 3 1)"#,
            r#"(string:right 3 1.1)"#,
            r#"(string:right 3 #t)"#,
            r#"(string:right 3 #f)"#,
            r#"(string:right 3 'symbol)"#,
            r#"(string:right 3 :keyword)"#,
            r#"(string:right 3 {:object-key 'value})"#,
            r#"(string:right 3 (cons 1 2))"#,
            r#"(string:right 3 #(+ %1 %2))"#,
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}

