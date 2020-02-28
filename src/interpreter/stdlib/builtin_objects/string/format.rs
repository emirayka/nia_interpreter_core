use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::stdlib::_lib;
use crate::interpreter::function::Function;

pub fn value_to_string(interpreter: &Interpreter, value: Value) -> Result<String, Error> {
    match value {
        Value::Integer(int) => Ok(int.to_string()),
        Value::Float(float) => Ok(float.to_string()),
        Value::Boolean(boolean) => if boolean {
            Ok(String::from("#t"))
        } else {
            Ok(String::from("#f"))
        },
        Value::String(string_id) => {
            let string = interpreter.get_string(string_id)?;
            
            Ok(String::from(string.get_string()))
        },
        Value::Symbol(symbol_id) => {
            let string = interpreter.get_symbol_name(symbol_id)?;

            Ok(String::from(string))
        },
        Value::Keyword(keyword_id) => {
            let keyword = interpreter.get_keyword(keyword_id)?;

            let mut string = String::from(":");
            string.push_str(keyword.get_name());

            Ok(string)
        },
        Value::Cons(cons_id) => {
            let values = interpreter.cons_to_vec(cons_id)?;

            let mut result = String::new();
            result.push_str("(");

            for value in values {
                result.push_str(&value_to_string(interpreter, value)?);
                result.push_str(" ");
            }

            result.remove(result.len() - 1);

            result.push_str(")");
            Ok(result)
        },
        Value::Object(object_id) => {
            let items = interpreter.get_items(object_id)?;

            let mut result = String::new();
            result.push_str("{");

            for (symbol_id, value) in items {
                let mut name = String::from(":");
                name.push_str(interpreter.get_symbol_name(*symbol_id)?);
                let string = value_to_string(interpreter, *value)?;

                result.push_str(&name);
                result.push_str(" ");
                result.push_str(&string);
                result.push_str(" ");
            }

            result.remove(result.len() - 1);
            result.push_str("}");

            Ok(result)
        },
        Value::Function(function_id) => {
            let function = interpreter.get_function(function_id)?;

            let string = match function {
                Function::Interpreted(_) => String::from("<function>"),
                Function::Builtin(_) => String::from("<builtin-function>"),
                Function::Macro(_) => String::from("<macro>"),
                Function::SpecialForm(_) => String::from("<special-form>"),
            };

            Ok(string)
        }
    }
}

pub fn format(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() < 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `string:format' takes at least one argument."
        ).into_result();
    }

    let mut values = values;

    let string = _lib::read_as_string(
        interpreter,
        values.remove(0)
    )?;

    let mut strings: Vec<&str> = string.split("{}").collect();

    if strings.len() - 1 != values.len() {
        return interpreter.make_invalid_argument_count_error(
            "Invalid count of arguments were provided."
        ).into_result();
    }

    let mut result = String::new();

    let mut iter_strings = strings.iter();
    let mut iter_values = values.iter();

    while let Some(value) = iter_values.next() {
        let s1 = iter_strings.next().unwrap();
        let s2 = value_to_string(interpreter, *value)?;

        result.push_str(s1);
        result.push_str(&s2);
    }

    let s = iter_strings.next().unwrap();
    result.push_str(s);

    Ok(interpreter.intern_string_value(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_correct_format_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(string:format "")"#,                                 r#""""#),
            (r#"(string:format "a")"#,                                r#""a""#),

            (r#"(string:format "{}" 1)"#,                             r#""1""#),
            (r#"(string:format "{}" 1.1)"#,                           r#""1.1""#),
            (r#"(string:format "{}" #t)"#,                            r##""#t""##),
            (r#"(string:format "{}" #f)"#,                            r##""#f""##),
            (r#"(string:format "{}" "string")"#,                      r#""string""#),
            (r#"(string:format "{}" 'symbol)"#,                       r#""symbol""#),
            (r#"(string:format "{}" :keyword)"#,                      r#"":keyword""#),
            (r#"(string:format "{}" '(a b c))"#,                      r#""(a b c)""#),
            (r#"(string:format "{}" {:key 'value})"#,                 r#""{:key value}""#),
            (r#"(string:format "{}" #(+ %1 %2))"#,                    r#""<function>""#),
            (r#"(string:format "{}" (flookup 'flookup))"#,            r#""<builtin-function>""#),
            (r#"(string:format "{}" (function (macro () 1)))"#,       r#""<macro>""#),
            (r#"(string:format "{}" (flookup 'cond))"#,               r#""<special-form>""#),

            (r#"(string:format "abc{}def" 1)"#,                       r#""abc1def""#),
            (r#"(string:format "abc{}def" 1.1)"#,                     r#""abc1.1def""#),
            (r#"(string:format "abc{}def" #t)"#,                      r#""abc#tdef""#),
            (r#"(string:format "abc{}def" #f)"#,                      r#""abc#fdef""#),
            (r#"(string:format "abc{}def" "string")"#,                r#""abcstringdef""#),
            (r#"(string:format "abc{}def" 'symbol)"#,                 r#""abcsymboldef""#),
            (r#"(string:format "abc{}def" :keyword)"#,                r#""abc:keyworddef""#),
            (r#"(string:format "abc{}def" '(a b c))"#,                r#""abc(a b c)def""#),
            (r#"(string:format "abc{}def" {:key 'value})"#,           r#""abc{:key value}def""#),
            (r#"(string:format "abc{}def" #(+ %1 %2))"#,              r#""abc<function>def""#),
            (r#"(string:format "abc{}def" (flookup 'flookup))"#,      r#""abc<builtin-function>def""#),
            (r#"(string:format "abc{}def" (function (macro () 1)))"#, r#""abc<macro>def""#),
            (r#"(string:format "abc{}def" (flookup 'cond))"#,         r#""abc<special-form>def""#),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_when_was_called_with_invalid_count_of_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            r#"(string:format)"#
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_when_first_argument_is_not_a_string() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
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
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_count_when_arguments_not_enough_to_format_string() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            r#"(string:format "{}")"#,
            r#"(string:format "{}" 1 2)"#,

            r#"(string:format "{} {}")"#,
            r#"(string:format "{} {}" 1)"#,
            r#"(string:format "{} {}" 1 2 3)"#,

            r#"(string:format "{} {} {}")"#,
            r#"(string:format "{} {} {}" 1)"#,
            r#"(string:format "{} {} {}" 1 2)"#,
            r#"(string:format "{} {} {}" 1 2 3 4)"#,
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
