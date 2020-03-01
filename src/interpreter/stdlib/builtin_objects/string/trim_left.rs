use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::lib;

pub fn trim_left(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `string:trim-left' takes only one argument."
        ).into_result();
    }

    let mut values = values;

    let string = lib::read_as_string(
        interpreter,
        values.remove(0)
    )?;

    let trimmed_string = String::from(string.trim_start());

    Ok(interpreter.intern_string_value(trimmed_string))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_correct_trimmed_from_left_string() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:trim-left " abc")"#, r#""abc""#),
            (r#"(string:trim-left "abc ")"#, r#""abc ""#),
            (r#"(string:trim-left " abc ")"#, r#""abc ""#),

            (r#"(string:trim-left "\n abc")"#, r#""abc""#),
            (r#"(string:trim-left "abc\n ")"#, r#""abc\n ""#),
            (r#"(string:trim-left "\n abc\n ")"#, r#""abc\n ""#),

            (r#"(string:trim-left "\r\n abc")"#, r#""abc""#),
            (r#"(string:trim-left "abc\r\n ")"#, r#""abc\r\n ""#),
            (r#"(string:trim-left "\r\n abc\r\n ")"#, r#""abc\r\n ""#),

            (r#"(string:trim-left "\r\n 猫猫猫")"#, r#""猫猫猫""#),
            (r#"(string:trim-left "猫猫猫\r\n ")"#, r#""猫猫猫\r\n ""#),
            (r#"(string:trim-left "\r\n 猫猫猫\r\n ")"#, r#""猫猫猫\r\n ""#),
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
            r#"(string:trim-left)"#,
            r#"(string:trim-left "a" "b")"#
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
            r#"(string:trim-left 1)"#,
            r#"(string:trim-left 1.1)"#,
            r#"(string:trim-left #t)"#,
            r#"(string:trim-left #f)"#,
            r#"(string:trim-left 'symbol)"#,
            r#"(string:trim-left :keyword)"#,
            r#"(string:trim-left {:object-key 'value})"#,
            r#"(string:trim-left (cons 1 2))"#,
            r#"(string:trim-left #(+ %1 %2))"#,
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}

