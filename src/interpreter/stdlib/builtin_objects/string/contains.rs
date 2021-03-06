use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;

pub fn contains(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `string:contains?' takes two strings exactly.",
        )
        .into();
    }

    let mut values = values;

    let string1 = library::read_as_string(interpreter, values.remove(0))?;

    let string2 = library::read_as_string(interpreter, values.remove(0))?;

    if string2.contains(string1) {
        Ok(Value::Boolean(true))
    } else {
        Ok(Value::Boolean(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_correct_comparison_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (r#"(string:contains? "" "abc")"#, Value::Boolean(true)),
            (r#"(string:contains? "a" "abc")"#, Value::Boolean(true)),
            (r#"(string:contains? "b" "abc")"#, Value::Boolean(true)),
            (r#"(string:contains? "c" "abc")"#, Value::Boolean(true)),
            (r#"(string:contains? "ab" "abc")"#, Value::Boolean(true)),
            (r#"(string:contains? "ac" "abc")"#, Value::Boolean(false)),
            (r#"(string:contains? "bc" "abc")"#, Value::Boolean(true)),
            (r#"(string:contains? "abc" "abc")"#, Value::Boolean(true)),
            (r#"(string:contains? "d" "abc")"#, Value::Boolean(false)),
            (r#"(string:contains? "abcc" "abc")"#, Value::Boolean(false)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_was_called_with_invalid_argument_count(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:contains?)"#,
            r#"(string:contains? "a")"#,
            r#"(string:contains? "a" "b" "c")"#,
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_when_was_called_with_not_strings() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:contains? "a" 1)"#,
            r#"(string:contains? "a" 1.1)"#,
            r#"(string:contains? "a" #t)"#,
            r#"(string:contains? "a" #f)"#,
            r#"(string:contains? "a" 'symbol)"#,
            r#"(string:contains? "a" :keyword)"#,
            r#"(string:contains? "a" {:object-key 'value})"#,
            r#"(string:contains? "a" (cons:new 1 2))"#,
            r#"(string:contains? "a" #(+ %1 %2))"#,
            r#"(string:contains? 1 "b")"#,
            r#"(string:contains? 1.1 "b")"#,
            r#"(string:contains? #t "b")"#,
            r#"(string:contains? #f "b")"#,
            r#"(string:contains? 'symbol "b")"#,
            r#"(string:contains? :keyword "b")"#,
            r#"(string:contains? {:object-key 'value} "b")"#,
            r#"(string:contains? (cons:new 1 2) "b")"#,
            r#"(string:contains? #(+ %1 %2) "b")"#,
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
