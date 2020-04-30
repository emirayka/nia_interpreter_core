use nom::{
    named,
    alt,
    tag,
    map_res,
    many1
};
use nom::branch::alt;
use nom::bytes::complete::tag;

use crate::parser::lib::parse_symbol_character;
use crate::parser::ParseError;

#[derive(Debug)]
pub struct SymbolElement {
    value: String,
}

impl SymbolElement {
    pub fn new(value: String) -> SymbolElement {
        SymbolElement {
             value
        }
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

    #[test]
    fn works_on_simple_value() {
        assert_eq!(Ok(("", SymbolElement {value: "test".to_string()})), parse("test"));
    }

    #[test]
    fn able_to_parse_all_fine_symbols() {
        let example = "test1-_^v=+?<>./&*%$@!~";
        assert_eq!(Ok(("", SymbolElement {value: String::from(example)})), parse(example));
    }

    #[test]
    fn able_to_parse_all_fine_escaped_symbols() {
        let text = r##"test\"\,\`\ \(\)\:\\\{\}"##;
        let expected = r##"test",` ():\{}"##;

        assert_eq!(Ok(("", SymbolElement {value: String::from(expected)})), parse(text));
    }

    #[test]
    fn allows_numbers_not_at_the_first_position() {
        assert_eq!(Ok(("", SymbolElement {value: String::from("test1")})), parse("test1"));
        assert_eq!(Ok(("", SymbolElement {value: String::from("1test")})), parse("1test"));
    }

    #[test]
    fn parses_special_symbols() {
        assert_eq!(Ok(("", SymbolElement {value: String::from("#opt")})), parse("#opt"));
        assert_eq!(Ok(("", SymbolElement {value: String::from("#rest")})), parse("#rest"));
        assert_eq!(Ok(("", SymbolElement {value: String::from("#keys")})), parse("#keys"));
    }

    #[test]
    fn does_not_parse_invalid_special_symbols() {
        assert!(parse("#tt").is_err());
    }
}
