use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;

pub fn lower(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `string:lower' takes only one argument.",
        )
        .into();
    }

    let mut values = values;

    let string = library::read_as_string(interpreter, values.remove(0))?;

    let lowercase_string = string.to_lowercase();

    Ok(interpreter.intern_string_value(&lowercase_string))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_correct_lowercase_string() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (r#"(string:lower "abc")"#, r#""abc""#),
            (r#"(string:lower "Abc")"#, r#""abc""#),
            (r#"(string:lower "aBc")"#, r#""abc""#),
            (r#"(string:lower "abC")"#, r#""abc""#),
            (r#"(string:lower "猫A钥")"#, r#""猫a钥""#),
            (r#"(string:lower "ὈΔΥΣΣΕΎΣ")"#, r#""ὀδυσσεύς""#),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_was_called_with_invalid_count_of_arguments(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector =
            vec![r#"(string:lower)"#, r#"(string:lower "a" "b")"#];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_was_called_with_invalid_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:lower 1)"#,
            r#"(string:lower 1.1)"#,
            r#"(string:lower #t)"#,
            r#"(string:lower #f)"#,
            r#"(string:lower 'symbol)"#,
            r#"(string:lower :keyword)"#,
            r#"(string:lower {:object-key 'value})"#,
            r#"(string:lower (cons:new 1 2))"#,
            r#"(string:lower #(+ %1 %2))"#,
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
