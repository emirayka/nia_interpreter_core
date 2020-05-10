use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;

pub fn format(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() == 0 {
        return Error::invalid_argument_count_error(
            "Built-in function `string:format' takes at least one argument.",
        )
        .into();
    }

    let result = library::_format(interpreter, values)?;

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
    fn returns_correct_format_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (r#"(string:format "")"#, r#""""#),
            (r#"(string:format "a")"#, r#""a""#),
            (r#"(string:format "{}" 1)"#, r#""1""#),
            (r#"(string:format "{}" 1.1)"#, r#""1.1""#),
            (r#"(string:format "{}" #t)"#, r##""#t""##),
            (r#"(string:format "{}" #f)"#, r##""#f""##),
            (r#"(string:format "{}" "string")"#, r#""string""#),
            (r#"(string:format "{}" 'symbol)"#, r#""symbol""#),
            (r#"(string:format "{}" :keyword)"#, r#"":keyword""#),
            (r#"(string:format "{}" '(a b c))"#, r#""(a b c)""#),
            (r#"(string:format "{}" {:key 'value})"#, r#""{:key value}""#),
            (r#"(string:format "{}" #(+ %1 %2))"#, r#""<function>""#),
            (
                r#"(string:format "{}" (flookup 'flookup))"#,
                r#""<builtin-function>""#,
            ),
            (
                r#"(string:format "{}" (function (macro () 1)))"#,
                r#""<macro>""#,
            ),
            (
                r#"(string:format "{}" (flookup 'cond))"#,
                r#""<special-form>""#,
            ),
            (r#"(string:format "abc{}def" 1)"#, r#""abc1def""#),
            (r#"(string:format "abc{}def" 1.1)"#, r#""abc1.1def""#),
            (r#"(string:format "abc{}def" #t)"#, r#""abc#tdef""#),
            (r#"(string:format "abc{}def" #f)"#, r#""abc#fdef""#),
            (
                r#"(string:format "abc{}def" "string")"#,
                r#""abcstringdef""#,
            ),
            (r#"(string:format "abc{}def" 'symbol)"#, r#""abcsymboldef""#),
            (
                r#"(string:format "abc{}def" :keyword)"#,
                r#""abc:keyworddef""#,
            ),
            (
                r#"(string:format "abc{}def" '(a b c))"#,
                r#""abc(a b c)def""#,
            ),
            (
                r#"(string:format "abc{}def" {:key 'value})"#,
                r#""abc{:key value}def""#,
            ),
            (
                r#"(string:format "abc{}def" #(+ %1 %2))"#,
                r#""abc<function>def""#,
            ),
            (
                r#"(string:format "abc{}def" (flookup 'flookup))"#,
                r#""abc<builtin-function>def""#,
            ),
            (
                r#"(string:format "abc{}def" (function (macro () 1)))"#,
                r#""abc<macro>def""#,
            ),
            (
                r#"(string:format "abc{}def" (flookup 'cond))"#,
                r#""abc<special-form>def""#,
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_when_was_called_with_invalid_count_of_arguments(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![r#"(string:format)"#];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_when_first_argument_is_not_a_string() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:format 1)"#,
            r#"(string:format 1.1)"#,
            r#"(string:format #t)"#,
            r#"(string:format #f)"#,
            r#"(string:format 'symbol)"#,
            r#"(string:format :keyword)"#,
            r#"(string:format '(list a b c))"#,
            r#"(string:format {:key 'value})"#,
            r#"(string:format #(+ %1 %2))"#,
            r#"(string:format (flookup 'flookup))"#,
            r#"(string:format (function (macro () 1)))"#,
            r#"(string:format (flookup 'cond))"#,
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_when_arguments_not_enough_to_format_string(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            r#"(string:format "{}")"#,
            r#"(string:format "{}" 1 2)"#,
            r#"(string:format "{} {}")"#,
            r#"(string:format "{} {}" 1)"#,
            r#"(string:format "{} {}" 1 2 3)"#,
            r#"(string:format "{} {} {}")"#,
            r#"(string:format "{} {} {}" 1)"#,
            r#"(string:format "{} {} {}" 1 2)"#,
            r#"(string:format "{} {} {}" 1 2 3 4)"#,
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
