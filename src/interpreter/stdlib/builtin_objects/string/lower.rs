use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::lib::_lib;

pub fn lower(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `string:lower' takes only one argument."
        ).into_result();
    }

    let mut values = values;

    let string = _lib::read_as_string(interpreter, values.remove(0))?;

    let lowercase_string = string.to_lowercase();

    Ok(interpreter.intern_string_value(lowercase_string))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_correct_lowercase_string() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:lower "abc")"#, r#""abc""#),

            (r#"(string:lower "Abc")"#, r#""abc""#),
            (r#"(string:lower "aBc")"#, r#""abc""#),
            (r#"(string:lower "abC")"#, r#""abc""#),

            (r#"(string:lower "猫A钥")"#, r#""猫a钥""#),
            (r#"(string:lower "ὈΔΥΣΣΕΎΣ")"#, r#""ὀδυσσεύς""#),
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
            r#"(string:lower)"#,
            r#"(string:lower "a" "b")"#
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
            r#"(string:lower 1)"#,
            r#"(string:lower 1.1)"#,
            r#"(string:lower #t)"#,
            r#"(string:lower #f)"#,
            r#"(string:lower 'symbol)"#,
            r#"(string:lower :keyword)"#,
            r#"(string:lower {:object-key 'value})"#,
            r#"(string:lower (cons 1 2))"#,
            r#"(string:lower #(+ %1 %2))"#,
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}

