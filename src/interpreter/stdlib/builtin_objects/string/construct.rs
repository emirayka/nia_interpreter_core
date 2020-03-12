use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;

pub fn construct(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() == 0 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `string:construct' takes one argument at least."
        ).into_result();
    }

    let values = values;
    let mut result = String::new();

    for value in values {
        let string = library::value_to_string(
            interpreter,
            value
        )?;

        result.push_str(&string);
    }

    Ok(interpreter.intern_string_value(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_constructed_string() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:construct "a")"#, r#""a""#),
            (r#"(string:construct "a" "b")"#, r#""ab""#),
            (r#"(string:construct "a" "b" "c")"#, r#""abc""#),

            (r#"(string:construct "a" 1)"#, r#""a1""#),
            (r#"(string:construct "a" 1.1)"#, r#""a1.1""#),
            (r#"(string:construct "a" #f)"#, r#""a#f""#),
            (r#"(string:construct "a" #t)"#, r#""a#t""#),
            (r#"(string:construct "a" "string")"#, r#""astring""#),
            (r#"(string:construct "a" 'symbol)"#, r#""asymbol""#),
            (r#"(string:construct "a" :keyword)"#, r#""a:keyword""#),
            (r#"(string:construct "a" '(1 2 3))"#, r#""a(1 2 3)""#),
            (r#"(string:construct "a" {})"#, r#""a{}""#),
            (r#"(string:construct "a" {:a 1})"#, r#""a{:a 1}""#),
            (r#"(string:construct "a" #())"#, r#""a<function>""#),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_when_was_called_with_invalid_amount_of_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(string:construct)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
