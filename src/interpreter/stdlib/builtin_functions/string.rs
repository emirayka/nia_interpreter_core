use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;

pub fn string(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
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
    fn returns_string() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string)"#, r#""""#),

            (r#"(string "a")"#, r#""a""#),
            (r#"(string "a" "b")"#, r#""ab""#),
            (r#"(string "a" "b" "c")"#, r#""abc""#),

            (r#"(string "a" 1)"#, r#""a1""#),
            (r#"(string "a" 1.1)"#, r#""a1.1""#),
            (r#"(string "a" #f)"#, r#""a#f""#),
            (r#"(string "a" #t)"#, r#""a#t""#),
            (r#"(string "a" "string")"#, r#""astring""#),
            (r#"(string "a" 'symbol)"#, r#""asymbol""#),
            (r#"(string "a" :keyword)"#, r#""a:keyword""#),
            (r#"(string "a" '(1 2 3))"#, r#""a(1 2 3)""#),
            (r#"(string "a" {})"#, r#""a{}""#),
            (r#"(string "a" {:a 1})"#, r#""a{:a 1}""#),
            (r#"(string "a" #())"#, r#""a<function>""#),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }
}
