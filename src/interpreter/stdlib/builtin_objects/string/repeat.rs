use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;

pub fn repeat(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `string:repeat' takes two arguments exactly."
        ).into();
    }

    let mut values = values;

    let count = library::read_as_i64(
        interpreter,
        values.remove(0)
    )?;

    if count < 0 {
        return Error::invalid_argument_error(
            "The first argument of built-in functino `string:repeat' must be a positive number"
        ).into();
    }

    let string = library::read_as_string(
        interpreter,
        values.remove(0)
    )?;
    
    let strings = std::iter::repeat(string)
        .take(count as usize)
        .collect::<Vec<&String>>();

    let mut result = String::new();

    for string in strings {
        result.push_str(string);
    }

    Ok(interpreter.intern_string_value(&result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn returns_repeated_string() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:repeat 0 "")"#, r#""""#),
            (r#"(string:repeat 1 "")"#, r#""""#),
            (r#"(string:repeat 2 "")"#, r#""""#),
            (r#"(string:repeat 3 "")"#, r#""""#),

            (r#"(string:repeat 0 "a")"#, r#""""#),
            (r#"(string:repeat 1 "a")"#, r#""a""#),
            (r#"(string:repeat 2 "a")"#, r#""aa""#),
            (r#"(string:repeat 3 "a")"#, r#""aaa""#),

            (r#"(string:repeat 0 "abc")"#, r#""""#),
            (r#"(string:repeat 1 "abc")"#, r#""abc""#),
            (r#"(string:repeat 2 "abc")"#, r#""abcabc""#),
            (r#"(string:repeat 3 "abc")"#, r#""abcabcabc""#),

            (r#"(string:repeat 0 "猫")"#, r#""""#),
            (r#"(string:repeat 1 "猫")"#, r#""猫""#),
            (r#"(string:repeat 2 "猫")"#, r#""猫猫""#),
            (r#"(string:repeat 3 "猫")"#, r#""猫猫猫""#),
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
            r#"(string:repeat)"#,
            r#"(string:repeat 3)"#,
            r#"(string:repeat 3 "b" "c")"#
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_was_called_with_negative_count() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            r#"(string:repeat -1 "test")"#,
            r#"(string:repeat -2 "test")"#,
            r#"(string:repeat -3 "test")"#,
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_was_called_with_invalid_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            r#"(string:repeat 1.1 "test")"#,
            r#"(string:repeat #t "test")"#,
            r#"(string:repeat #f "test")"#,
            r#"(string:repeat "test" "test")"#,
            r#"(string:repeat 'symbol "test")"#,
            r#"(string:repeat :keyword "test")"#,
            r#"(string:repeat {:object-key 'value} "test")"#,
            r#"(string:repeat (cons 1 2) "test")"#,
            r#"(string:repeat #(+ %1 %2) "test")"#,

            r#"(string:repeat 3 1)"#,
            r#"(string:repeat 3 1.1)"#,
            r#"(string:repeat 3 #t)"#,
            r#"(string:repeat 3 #f)"#,
            r#"(string:repeat 3 'symbol)"#,
            r#"(string:repeat 3 :keyword)"#,
            r#"(string:repeat 3 {:object-key 'value})"#,
            r#"(string:repeat 3 (cons 1 2))"#,
            r#"(string:repeat 3 #(+ %1 %2))"#,
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}

