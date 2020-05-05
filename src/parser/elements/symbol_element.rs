use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::{alt, many1, map_res, named, tag};

use crate::parser::lib::parse_symbol_character;
use crate::parser::ParseError;

#[derive(Debug, Clone)]
pub struct SymbolElement {
    value: String,
}

impl SymbolElement {
    pub fn new(value: String) -> SymbolElement {
        SymbolElement { value }
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}

impl PartialEq for SymbolElement {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for SymbolElement {}

fn join(chars: Vec<char>) -> Result<String, String> {
    Ok(chars.iter().collect())
}

fn str_to_string(string: &str) -> Result<String, ParseError> {
    Ok(String::from(string))
}

fn make_symbol_element(value: String) -> Result<SymbolElement, ParseError> {
    Ok(SymbolElement::new(value))
}

named!(parse_special_symbols(&str) -> String, map_res!(
    alt!(
        tag!("#opt") |
        tag!("#rest") |
        tag!("#keys")
    ),
    str_to_string
));

named!(parse_ordinary_symbol(&str) -> String, map_res!(
    many1!(parse_symbol_character),
    join
));

named!(pub parse(&str) -> SymbolElement, map_res!(
    alt!(
        parse_special_symbols |
        parse_ordinary_symbol
    ),
    make_symbol_element
));

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn works_on_simple_value() {
        nia_assert_equal(
            Ok((
                "",
                SymbolElement {
                    value: "test".to_string(),
                },
            )),
            parse("test"),
        );
    }

    #[test]
    fn able_to_parse_all_fine_symbols() {
        let example = "test1-_^v=+?<>./&*%$@!~";
        nia_assert_equal(
            Ok((
                "",
                SymbolElement {
                    value: String::from(example),
                },
            )),
            parse(example),
        );
    }

    #[test]
    fn able_to_parse_all_fine_escaped_symbols() {
        let text = r##"test\"\,\`\ \(\)\:\\\{\}"##;
        let expected = r##"test",` ():\{}"##;

        nia_assert_equal(
            Ok((
                "",
                SymbolElement {
                    value: String::from(expected),
                },
            )),
            parse(text),
        );
    }

    #[test]
    fn allows_numbers_not_at_the_first_position() {
        nia_assert_equal(
            Ok((
                "",
                SymbolElement {
                    value: String::from("test1"),
                },
            )),
            parse("test1"),
        );
        nia_assert_equal(
            Ok((
                "",
                SymbolElement {
                    value: String::from("1test"),
                },
            )),
            parse("1test"),
        );
    }

    #[test]
    fn parses_special_symbols() {
        nia_assert_equal(
            Ok((
                "",
                SymbolElement {
                    value: String::from("#opt"),
                },
            )),
            parse("#opt"),
        );
        nia_assert_equal(
            Ok((
                "",
                SymbolElement {
                    value: String::from("#rest"),
                },
            )),
            parse("#rest"),
        );
        nia_assert_equal(
            Ok((
                "",
                SymbolElement {
                    value: String::from("#keys"),
                },
            )),
            parse("#keys"),
        );
    }

    #[test]
    fn does_not_parse_invalid_special_symbols() {
        nia_assert(parse("#tt").is_err());
    }
}
