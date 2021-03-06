use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;

pub fn join(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() < 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `string:join' takes two arguments at least.",
        )
        .into();
    }

    let mut values = values;

    let joiner = library::read_as_string(interpreter, values.remove(0))?;

    let values = if values.len() == 1 {
        match values.remove(0) {
            Value::Cons(cons_id) => {
                interpreter.list_to_vec(cons_id)
                    .map_err(|err| Error::generic_execution_error_caused(
                        "",
                        err
                    ))?
            },
            Value::Symbol(symbol_id) => {
                if interpreter.symbol_is_nil(symbol_id)? {
                    Vec::new()
                } else {
                    return Error::invalid_argument_error(
                        "If built-in function `string:join' was called with two arguments, the latter must be a cons or string."
                    ).into();
                }
            },
            value @ Value::String(_) => vec!(value),
            _ => return Error::invalid_argument_error(
                "If built-in function `string:join' was called with two arguments, the latter must be a cons or string."
            ).into()
        }
    } else {
        values
    };

    let mut result = String::new();

    for value in values {
        let string = library::read_as_string(interpreter, value)?;

        result.push_str(string);
        result.push_str(joiner);
    }

    if result.len() > 0 && joiner.len() > 0 {
        result.replace_range((result.len() - joiner.len()).., "");
    }

    Ok(interpreter.intern_string_value(&result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_correct_join_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
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
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_when_was_called_with_invalid_amount_of_arguments(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![r#"(string:join)"#, r#"(string:join "|")"#];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_when_was_called_with_not_strings() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:join "|" 1)"#,
            r#"(string:join "|" 1.1)"#,
            r#"(string:join "|" #t)"#,
            r#"(string:join "|" #f)"#,
            r#"(string:join "|" 'symbol)"#,
            r#"(string:join "|" :keyword)"#,
            r#"(string:join "|" {:object-key 'value})"#,
            r#"(string:join "|" (cons:new 1 2))"#,
            r#"(string:join "|" #(+ %1 %2))"#,
            r#"(string:join "|" '(1))"#,
            r#"(string:join "|" '(1.1))"#,
            r#"(string:join "|" '(#t))"#,
            r#"(string:join "|" '(#f))"#,
            r#"(string:join "|" '(symbol))"#,
            r#"(string:join "|" '(:keyword))"#,
            r#"(string:join "|" '({:object-key 'value}))"#,
            r#"(string:join "|" (list:new (cons:new 1 2)))"#,
            r#"(string:join "|" (list:new #(+ %1 %2)))"#,
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
