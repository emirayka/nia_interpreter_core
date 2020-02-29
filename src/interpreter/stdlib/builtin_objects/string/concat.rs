use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::lib::_lib;

pub fn concat(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() == 0 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `string:concat' takes two arguments exactly."
        ).into_result();
    }

    let mut values = values;

    let first_string = _lib::read_as_string(
        interpreter,
        values.remove(0)
    )?;

    let mut result = String::from(first_string);

    loop {
        if values.len() == 0 {
            break;
        }

        let next_string = _lib::read_as_string(
            interpreter,
            values.remove(0)
        )?;

        result.push_str(next_string);
    }

    Ok(interpreter.intern_string_value(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_concatenated_strings() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:concat "a")"#, interpreter.intern_string_value(String::from("a"))),
            (r#"(string:concat "a" "b")"#, interpreter.intern_string_value(String::from("ab"))),
            (r#"(string:concat "a" "b" "c")"#, interpreter.intern_string_value(String::from("abc"))),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_when_was_called_with_invalid_amount_of_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(string:concat)"
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
            r#"(string:concat 1)"#,
            r#"(string:concat 1.1)"#,
            r#"(string:concat #t)"#,
            r#"(string:concat #f)"#,
            r#"(string:concat 'symbol)"#,
            r#"(string:concat :keyword)"#,
            r#"(string:concat {:object-key 'value})"#,
            r#"(string:concat (cons 1 2))"#,
            r#"(string:concat #(+ %1 %2))"#,

            r#"(string:concat "a" 1)"#,
            r#"(string:concat "a" 1.1)"#,
            r#"(string:concat "a" #t)"#,
            r#"(string:concat "a" #f)"#,
            r#"(string:concat "a" 'symbol)"#,
            r#"(string:concat "a" :keyword)"#,
            r#"(string:concat "a" {:object-key 'value})"#,
            r#"(string:concat "a" (cons 1 2))"#,
            r#"(string:concat "a" #(+ %1 %2))"#,
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}

