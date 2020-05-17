use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::library;

pub fn _format(
    interpreter: &mut Interpreter,
    values: Vec<Value>,
) -> Result<String, Error> {
    let mut values = values;

    let string = library::read_as_string(interpreter, values.remove(0))?;

    let strings: Vec<&str> = string.split("{}").collect();

    if strings.len() - 1 != values.len() {
        return Error::invalid_argument_count_error(
            "Invalid count of arguments were provided.",
        )
        .into();
    }

    let mut result = String::new();

    let mut iter_strings = strings.iter();
    let mut iter_values = values.iter();

    while let Some(value) = iter_values.next() {
        let s1 = iter_strings.next().unwrap();
        let s2 = library::value_to_string(interpreter, *value)?;

        result.push_str(s1);
        result.push_str(&s2);
    }

    let s = iter_strings.next().unwrap();
    result.push_str(s);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use std::convert::TryInto;

    #[test]
    fn returns_correct_format_result() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (r#"(list:new "")"#, r#""#),
            (r#"(list:new "a")"#, r#"a"#),
            (r#"(list:new "{}" 1)"#, r#"1"#),
            (r#"(list:new "{}" 1.1)"#, r#"1.1"#),
            (r#"(list:new "{}" #t)"#, r##"#t"##),
            (r#"(list:new "{}" #f)"#, r##"#f"##),
            (r#"(list:new "{}" "string")"#, r#"string"#),
            (r#"(list:new "{}" 'symbol)"#, r#"symbol"#),
            (r#"(list:new "{}" :keyword)"#, r#":keyword"#),
            (r#"(list:new "{}" '(a b c))"#, r#"(a b c)"#),
            (r#"(list:new "{}" {:key 'value})"#, r#"{:key value}"#),
            (r#"(list:new "{}" #(+ %1 %2))"#, r#"<function>"#),
            (r#"(list:new "{}" (flookup 'flookup))"#, r#"<builtin-function>"#),
            (r#"(list:new "{}" (function (macro () 1)))"#, r#"<macro>"#),
            (r#"(list:new "{}" (flookup 'cond))"#, r#"<special-form>"#),
            (r#"(list:new "abc{}def" 1)"#, r#"abc1def"#),
            (r#"(list:new "abc{}def" 1.1)"#, r#"abc1.1def"#),
            (r#"(list:new "abc{}def" #t)"#, r#"abc#tdef"#),
            (r#"(list:new "abc{}def" #f)"#, r#"abc#fdef"#),
            (r#"(list:new "abc{}def" "string")"#, r#"abcstringdef"#),
            (r#"(list:new "abc{}def" 'symbol)"#, r#"abcsymboldef"#),
            (r#"(list:new "abc{}def" :keyword)"#, r#"abc:keyworddef"#),
            (r#"(list:new "abc{}def" '(a b c))"#, r#"abc(a b c)def"#),
            (
                r#"(list:new "abc{}def" {:key 'value})"#,
                r#"abc{:key value}def"#,
            ),
            (r#"(list:new "abc{}def" #(+ %1 %2))"#, r#"abc<function>def"#),
            (
                r#"(list:new "abc{}def" (flookup 'flookup))"#,
                r#"abc<builtin-function>def"#,
            ),
            (
                r#"(list:new "abc{}def" (function (macro () 1)))"#,
                r#"abc<macro>def"#,
            ),
            (
                r#"(list:new "abc{}def" (flookup 'cond))"#,
                r#"abc<special-form>def"#,
            ),
        ];

        for (code, expected) in specs {
            let value = interpreter.execute_in_main_environment(code).unwrap();
            let cons_id = value.try_into().unwrap();
            let values = interpreter.list_to_vec(cons_id).unwrap();

            let result = _format(&mut interpreter, values).unwrap();

            nia_assert_equal(expected, &result)
        }
    }
}
