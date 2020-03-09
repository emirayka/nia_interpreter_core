use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;

pub fn split(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `string:split' takes only one argument."
        ).into_result();
    }

    let mut values = values;

    let splitted = {
        let separator = library::read_as_string(
            interpreter,
            values.remove(0)
        )?;

        let string = library::read_as_string(
            interpreter,
            values.remove(0)
        )?;

        string.split(separator)
            .map(|s| String::from(s))
            .collect::<Vec<String>>()
    };

    let mut values = Vec::new();

    for string in splitted {
        values.push(interpreter.intern_string_value(string));
    }

    let cons = interpreter.vec_to_list(values);

    Ok(cons)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_list_of_splitted_strings() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:split "" "")"#,            r#"'("" "")"#),
            (r#"(string:split "" "a")"#,           r#"'("" "a" "")"#),
            (r#"(string:split "" "a|b")"#,         r#"'("" "a" "|" "b" "")"#),
            (r#"(string:split "" "a|b|c")"#,       r#"'("" "a" "|" "b" "|" "c" "")"#),

            (r#"(string:split "|" "")"#,           r#"'("")"#),
            (r#"(string:split "|" "a")"#,          r#"'("a")"#),
            (r#"(string:split "|" "a|b")"#,        r#"'("a" "b")"#),
            (r#"(string:split "|" "a|b|c")"#,      r#"'("a" "b" "c")"#),

            (r#"(string:split "猫" "猫a钥b匙c月")"#, r#"'("" "a钥b匙c月")"#),
            (r#"(string:split "a" "猫a钥b匙c月")"#,  r#"'("猫" "钥b匙c月")"#),
            (r#"(string:split "钥" "猫a钥b匙c月")"#, r#"'("猫a" "b匙c月")"#),
            (r#"(string:split "b" "猫a钥b匙c月")"#,  r#"'("猫a钥" "匙c月")"#),
            (r#"(string:split "匙" "猫a钥b匙c月")"#, r#"'("猫a钥b" "c月")"#),
            (r#"(string:split "c" "猫a钥b匙c月")"#,  r#"'("猫a钥b匙" "月")"#),
            (r#"(string:split "月" "猫a钥b匙c月")"#, r#"'("猫a钥b匙c" "")"#),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_was_called_with_invalid_argument_count() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            r#"(string:split)"#,
            r#"(string:split "a")"#,
            r#"(string:split "a" "b" "c")"#
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
            r#"(string:split "a" 1)"#,
            r#"(string:split "a" 1.1)"#,
            r#"(string:split "a" #t)"#,
            r#"(string:split "a" #f)"#,
            r#"(string:split "a" 'symbol)"#,
            r#"(string:split "a" :keyword)"#,
            r#"(string:split "a" {:object-key 'value})"#,
            r#"(string:split "a" (cons 1 2))"#,
            r#"(string:split "a" #(+ %1 %2))"#,

            r#"(string:split 1 "b")"#,
            r#"(string:split 1.1 "b")"#,
            r#"(string:split #t "b")"#,
            r#"(string:split #f "b")"#,
            r#"(string:split 'symbol "b")"#,
            r#"(string:split :keyword "b")"#,
            r#"(string:split {:object-key 'value} "b")"#,
            r#"(string:split (cons 1 2) "b")"#,
            r#"(string:split #(+ %1 %2) "b")"#,
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}

