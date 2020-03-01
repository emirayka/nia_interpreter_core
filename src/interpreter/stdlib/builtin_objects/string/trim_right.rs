use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::lib;

pub fn trim_right(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `string:trim-right' takes only one argument."
        ).into_result();
    }

    let mut values = values;

    let string = lib::read_as_string(
        interpreter,
        values.remove(0)
    )?;

    let trimmed_string = String::from(string.trim_end());

    Ok(interpreter.intern_string_value(trimmed_string))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_correct_trimmed_from_right_string() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:trim-right " abc")"#, r#"" abc""#),
            (r#"(string:trim-right "abc ")"#, r#""abc""#),
            (r#"(string:trim-right " abc ")"#, r#"" abc""#),

            (r#"(string:trim-right "\n abc")"#, r#""\n abc""#),
            (r#"(string:trim-right "abc\n ")"#, r#""abc""#),
            (r#"(string:trim-right "\n abc\n ")"#, r#""\n abc""#),

            (r#"(string:trim-right "\r\n abc")"#, r#""\r\n abc""#),
            (r#"(string:trim-right "abc\r\n ")"#, r#""abc""#),
            (r#"(string:trim-right "\r\n abc\r\n ")"#, r#""\r\n abc""#),

            (r#"(string:trim-right "\r\n 猫猫猫")"#, r#""\r\n 猫猫猫""#),
            (r#"(string:trim-right "猫猫猫\r\n ")"#, r#""猫猫猫""#),
            (r#"(string:trim-right "\r\n 猫猫猫\r\n ")"#, r#""\r\n 猫猫猫""#),
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
            r#"(string:trim-right)"#,
            r#"(string:trim-right "a" "b")"#
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
            r#"(string:trim-right 1)"#,
            r#"(string:trim-right 1.1)"#,
            r#"(string:trim-right #t)"#,
            r#"(string:trim-right #f)"#,
            r#"(string:trim-right 'symbol)"#,
            r#"(string:trim-right :keyword)"#,
            r#"(string:trim-right {:object-key 'value})"#,
            r#"(string:trim-right (cons 1 2))"#,
            r#"(string:trim-right #(+ %1 %2))"#,
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}

