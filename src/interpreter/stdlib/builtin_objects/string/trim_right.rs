use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;

pub fn trim_right(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `string:trim-right' takes only one argument.",
        )
        .into();
    }

    let mut values = values;

    let string = library::read_as_string(interpreter, values.remove(0))?;

    let trimmed_string = String::from(string.trim_end());

    Ok(interpreter.intern_string_value(&trimmed_string))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_correct_trimmed_from_right_string() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
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
            (
                r#"(string:trim-right "\r\n 猫猫猫\r\n ")"#,
                r#""\r\n 猫猫猫""#,
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_was_called_with_invalid_count_of_arguments(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector =
            vec![r#"(string:trim-right)"#, r#"(string:trim-right "a" "b")"#];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_was_called_with_invalid_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:trim-right 1)"#,
            r#"(string:trim-right 1.1)"#,
            r#"(string:trim-right #t)"#,
            r#"(string:trim-right #f)"#,
            r#"(string:trim-right 'symbol)"#,
            r#"(string:trim-right :keyword)"#,
            r#"(string:trim-right {:object-key 'value})"#,
            r#"(string:trim-right (cons:new 1 2))"#,
            r#"(string:trim-right #(+ %1 %2))"#,
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
