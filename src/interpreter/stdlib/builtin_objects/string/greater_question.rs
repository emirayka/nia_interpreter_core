use std::cmp::Ordering;

use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::lib;

pub fn greater_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `string:greater?' takes two arguments exactly."
        ).into_result();
    }

    let mut values = values;

    let string1 = lib::read_as_string(interpreter, values.remove(0))?;
    let string2 = lib::read_as_string(interpreter, values.remove(0))?;

    let result = match string1.cmp(string2) {
        Ordering::Less => Value::Boolean(false),
        Ordering::Equal => Value::Boolean(false),
        Ordering::Greater => Value::Boolean(true),
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_correct_comparison_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:greater? "" "a")"#,  Value::Boolean(false)),
            (r#"(string:greater? "a" "")"#,  Value::Boolean(true)),

            (r#"(string:greater? "abc" "abc")"#, Value::Boolean(false)),
            (r#"(string:greater? "abc" "def")"#, Value::Boolean(false)),
            (r#"(string:greater? "def" "def")"#, Value::Boolean(false)),
            (r#"(string:greater? "def" "abc")"#,  Value::Boolean(true)),

            (r#"(string:greater? "abc" "ab")"#,  Value::Boolean(true)),
            (r#"(string:greater? "abc" "de")"#,  Value::Boolean(false)),
            (r#"(string:greater? "ab" "abc")"#,  Value::Boolean(false)),
            (r#"(string:greater? "de" "abc")"#,  Value::Boolean(true)),

            (r#"(string:greater? "" "")"#,  Value::Boolean(false)),
            (r#"(string:greater? "a" "a")"#,  Value::Boolean(false)),
            (r#"(string:greater? "ab" "ab")"#,  Value::Boolean(false)),
            (r#"(string:greater? "abc" "abc")"#,  Value::Boolean(false))
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_was_called_with_invalid_argument_count() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            r#"(string:greater?)"#,
            r#"(string:greater? "a")"#,
            r#"(string:greater? "a" "b" "c")"#
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_when_was_called_with_not_strings() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            r#"(string:greater? "a" 1)"#,
            r#"(string:greater? "a" 1.1)"#,
            r#"(string:greater? "a" #t)"#,
            r#"(string:greater? "a" #f)"#,
            r#"(string:greater? "a" 'symbol)"#,
            r#"(string:greater? "a" :keyword)"#,
            r#"(string:greater? "a" {:object-key 'value})"#,
            r#"(string:greater? "a" (cons 1 2))"#,
            r#"(string:greater? "a" #(+ %1 %2))"#,

            r#"(string:greater? 1 "b")"#,
            r#"(string:greater? 1.1 "b")"#,
            r#"(string:greater? #t "b")"#,
            r#"(string:greater? #f "b")"#,
            r#"(string:greater? 'symbol "b")"#,
            r#"(string:greater? :keyword "b")"#,
            r#"(string:greater? {:object-key 'value} "b")"#,
            r#"(string:greater? (cons 1 2) "b")"#,
            r#"(string:greater? #(+ %1 %2) "b")"#,
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
