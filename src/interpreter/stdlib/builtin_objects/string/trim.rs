use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;

pub fn trim(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `string:trim' takes only one argument.",
        )
        .into();
    }

    let mut values = values;

    let string = library::read_as_string(interpreter, values.remove(0))?;

    let trimmed_string = String::from(string.trim());

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
    fn returns_correct_trimmed_string() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (r#"(string:trim " abc")"#, r#""abc""#),
            (r#"(string:trim "abc ")"#, r#""abc""#),
            (r#"(string:trim " abc ")"#, r#""abc""#),
            (r#"(string:trim "\n abc")"#, r#""abc""#),
            (r#"(string:trim "abc\n ")"#, r#""abc""#),
            (r#"(string:trim "\n abc\n ")"#, r#""abc""#),
            (r#"(string:trim "\r\n abc")"#, r#""abc""#),
            (r#"(string:trim "abc\r\n ")"#, r#""abc""#),
            (r#"(string:trim "\r\n abc\r\n ")"#, r#""abc""#),
            (r#"(string:trim "\r\n 猫猫猫")"#, r#""猫猫猫""#),
            (r#"(string:trim "猫猫猫\r\n ")"#, r#""猫猫猫""#),
            (r#"(string:trim "\r\n 猫猫猫\r\n ")"#, r#""猫猫猫""#),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_was_called_with_invalid_count_of_arguments(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![r#"(string:trim)"#, r#"(string:trim "a" "b")"#];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_was_called_with_invalid_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:trim 1)"#,
            r#"(string:trim 1.1)"#,
            r#"(string:trim #t)"#,
            r#"(string:trim #f)"#,
            r#"(string:trim 'symbol)"#,
            r#"(string:trim :keyword)"#,
            r#"(string:trim {:object-key 'value})"#,
            r#"(string:trim (cons 1 2))"#,
            r#"(string:trim #(+ %1 %2))"#,
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
