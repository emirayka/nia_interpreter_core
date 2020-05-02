use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;

pub fn upper(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `string:upper' takes only one argument."
        ).into();
    }

    let mut values = values;

    let string = library::read_as_string(
        interpreter,
        values.remove(0)
    )?;

    let uppercase_string = string.to_uppercase();

    Ok(interpreter.intern_string_value(&uppercase_string))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_uppercase_string() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:upper "abc")"#, r#""ABC""#),

            (r#"(string:upper "Abc")"#, r#""ABC""#),
            (r#"(string:upper "aBc")"#, r#""ABC""#),
            (r#"(string:upper "abC")"#, r#""ABC""#),

            (r#"(string:upper "猫a钥")"#, r#""猫A钥""#),
            (r#"(string:upper "ὀδυσσεύς")"#, r#""ὈΔΥΣΣΕΎΣ""#),
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
            r#"(string:upper)"#,
            r#"(string:upper "a" "b")"#
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
            r#"(string:upper 1)"#,
            r#"(string:upper 1.1)"#,
            r#"(string:upper #t)"#,
            r#"(string:upper #f)"#,
            r#"(string:upper 'symbol)"#,
            r#"(string:upper :keyword)"#,
            r#"(string:upper {:object-key 'value})"#,
            r#"(string:upper (cons 1 2))"#,
            r#"(string:upper #(+ %1 %2))"#,
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}

