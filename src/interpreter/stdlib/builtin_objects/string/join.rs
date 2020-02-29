use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::stdlib::_lib;

pub fn join(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() < 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `string:join' takes two arguments at least."
        ).into_result();
    }

    let mut values = values;

    let joiner = _lib::read_as_string(
        interpreter,
        values.remove(0)
    )?;

    let values = if values.len() == 1 {
        match values.remove(0) {
            Value::Cons(cons_id) => {
                interpreter.cons_to_vec(cons_id)
                    .map_err(|err| interpreter.make_generic_execution_error_caused(
                        "",
                        err
                    ))?
            },
            Value::Symbol(symbol_id) => {
                let symbol = interpreter.get_symbol(symbol_id)
                    .map_err(|err| interpreter.make_generic_execution_error_caused(
                        "",
                        err
                    ))?;

                if symbol.is_nil() {
                    Vec::new()
                } else {
                    return interpreter.make_invalid_argument_error(
                        "If built-in function `string:join' was called with two arguments, the latter must be a cons or string."
                    ).into_result();
                }
            },
            value @ Value::String(_) => vec!(value),
            _ => return interpreter.make_invalid_argument_error(
                "If built-in function `string:join' was called with two arguments, the latter must be a cons or string."
            ).into_result()
        }
    } else {
        values
    };

    let mut result = String::new();

    for value in values {
        let string = _lib::read_as_string(interpreter, value)?;

        result.push_str(string);
        result.push_str(joiner);
    }

    if result.len() > 0 && joiner.len() > 0 {
        result.replace_range((result.len() - joiner.len()).., "");
    }

    Ok(interpreter.intern_string_value(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_correct_join_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:join "" "b")"#, r#""b""#),
            (r#"(string:join "" "b" "c")"#, r#""bc""#),
            (r#"(string:join "" "b" "c" "d")"#, r#""bcd""#),

            (r#"(string:join "" '())"#, r#""""#),
            (r#"(string:join "|" '())"#, r#""""#),
            (r#"(string:join "||" '())"#, r#""""#),

            (r#"(string:join "|" "b")"#, r#""b""#),
            (r#"(string:join "|" "b" "c")"#, r#""b|c""#),
            (r#"(string:join "|" "b" "c" "d")"#, r#""b|c|d""#),

            (r#"(string:join "|" '("b"))"#, r#""b""#),
            (r#"(string:join "|" '("b" "c"))"#, r#""b|c""#),
            (r#"(string:join "|" '("b" "c" "d"))"#, r#""b|c|d""#),

            (r#"(string:join "||" "b")"#, r#""b""#),
            (r#"(string:join "||" "b" "c")"#, r#""b||c""#),
            (r#"(string:join "||" "b" "c" "d")"#, r#""b||c||d""#),
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
            r#"(string:join)"#,
            r#"(string:join "|")"#
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
            r#"(string:join "|" 1)"#,
            r#"(string:join "|" 1.1)"#,
            r#"(string:join "|" #t)"#,
            r#"(string:join "|" #f)"#,
            r#"(string:join "|" 'symbol)"#,
            r#"(string:join "|" :keyword)"#,
            r#"(string:join "|" {:object-key 'value})"#,
            r#"(string:join "|" (cons 1 2))"#,
            r#"(string:join "|" #(+ %1 %2))"#,

            r#"(string:join "|" '(1))"#,
            r#"(string:join "|" '(1.1))"#,
            r#"(string:join "|" '(#t))"#,
            r#"(string:join "|" '(#f))"#,
            r#"(string:join "|" '(symbol))"#,
            r#"(string:join "|" '(:keyword))"#,
            r#"(string:join "|" '({:object-key 'value}))"#,
            r#"(string:join "|" (list (cons 1 2)))"#,
            r#"(string:join "|" (list #(+ %1 %2)))"#,
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}

