use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;

pub fn length(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `string:len' takes only one argument."
        ).into();
    }

    let mut values = values;

    let string = library::read_as_string(
        interpreter,
        values.remove(0)
    )?;

    let length = string.chars().count() as i64;

    Ok(Value::Integer(length))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_length_of_string_in_utf8_chars() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:length "")"#,      r#"0"#),
            (r#"(string:length "a")"#,     r#"1"#),
            (r#"(string:length "ab")"#,    r#"2"#),
            (r#"(string:length "abc")"#,   r#"3"#),

            (r#"(string:length "猫")"#,    r#"1"#),
            (r#"(string:length "猫a")"#,   r#"2"#),
            (r#"(string:length "猫a钥")"#, r#"3"#),
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
            r#"(string:length)"#,
            r#"(string:length "a" "b")"#
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
            r#"(string:length 1)"#,
            r#"(string:length 1.1)"#,
            r#"(string:length #t)"#,
            r#"(string:length #f)"#,
            r#"(string:length 'symbol)"#,
            r#"(string:length :keyword)"#,
            r#"(string:length {:object-key 'value})"#,
            r#"(string:length (cons 1 2))"#,
            r#"(string:length #(+ %1 %2))"#,
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}

