use nom::alt;
use nom::delimited;
use nom::many0;
use nom::map_res;
use nom::named;
use nom::none_of;
use nom::tag;

use crate::interpreter::parser::ParseError;

#[derive(Debug, Clone)]
pub struct StringElement {
    value: String,
}

impl StringElement {
    pub fn new(value: String) -> StringElement {
        StringElement { value }
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}

impl PartialEq for StringElement {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for StringElement {}

fn make_slash_char(_: &str) -> Result<char, ParseError> {
    Ok('\\')
}

fn make_quotation_char(_: &str) -> Result<char, ParseError> {
    Ok('\"')
}

fn make_eol_char(_: &str) -> Result<char, ParseError> {
    Ok('\n')
}

fn make_carriage_return_char(_: &str) -> Result<char, ParseError> {
    Ok('\r')
}

fn collect_chars(chars: Vec<char>) -> Result<String, ParseError> {
    Ok(chars.into_iter().collect::<String>())
}

fn make_string_element(value: String) -> Result<StringElement, ParseError> {
    Ok(StringElement { value })
}

named!(parse_escaped_slash(&str) -> char, map_res!(
    tag!("\\\\"), make_slash_char
));

named!(parse_escaped_quotation(&str) -> char, map_res!(
    tag!("\\\""), make_quotation_char
));

named!(parse_escaped_eol(&str) -> char, map_res!(
    tag!("\\n"), make_eol_char
));

named!(parse_escaped_carriage_return(&str) -> char, map_res!(
    tag!("\\r"), make_carriage_return_char
));

named!(parse_escaped_character(&str) -> char, alt!(
    parse_escaped_slash |
    parse_escaped_quotation |
    parse_escaped_eol |
    parse_escaped_carriage_return
));

named!(parse_not_escaped_character(&str) -> char, none_of!("\\\""));

named!(parse_inner_character(&str) -> char, alt!(
    parse_escaped_character |
    parse_not_escaped_character
));

named!(parse_inner_characters(&str) -> String, map_res!(
    many0!(parse_inner_character),
    collect_chars
));

named!(parse_string(&str) -> String, delimited!(
    tag!("\""),
    parse_inner_characters,
    tag!("\"")
));

named!(pub parse(&str) -> StringElement, map_res!(parse_string, make_string_element));

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn works_on_simple_values() {
        nia_assert_equal(
            Ok((
                "",
                StringElement {
                    value: r"test".to_string(),
                },
            )),
            parse(r#""test""#),
        );
    }

    #[test]
    fn escape_behaves_correctly() {
        nia_assert_equal(
            Ok((
                "",
                StringElement {
                    value: "\\".to_string(),
                },
            )),
            parse(r#""\\""#),
        );
        nia_assert_equal(
            Ok((
                "",
                StringElement {
                    value: "\"".to_string(),
                },
            )),
            parse(r#""\"""#),
        );
        nia_assert_equal(
            Ok((
                "",
                StringElement {
                    value: "\n".to_string(),
                },
            )),
            parse(r#""\n""#),
        );
        nia_assert_equal(
            Ok((
                "",
                StringElement {
                    value: "\r".to_string(),
                },
            )),
            parse(r#""\r""#),
        );
        nia_assert_equal(
            Ok((
                "",
                StringElement {
                    value: "knock\"knockknock".to_string(),
                },
            )),
            parse(r#""knock\"knockknock""#),
        );
    }
}
