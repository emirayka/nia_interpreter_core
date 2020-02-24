use nom::{
    combinator::map_res,
    multi::many1,
};
use crate::parser::lib::parse_symbol_character;
use nom::branch::alt;
use nom::bytes::complete::tag;

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

fn str_to_string(string: &str) -> Result<String, String> {
    Ok(String::from(string))
}

fn make_symbol_element(value: String) -> Result<SymbolElement, String> {
    Ok(SymbolElement::new(value))
}

pub fn parse_symbol_element(s: &str) -> Result<(&str, SymbolElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse_special_symbols = alt((
        tag("#opt"),
        tag("#rest"),
        tag("#keys"),
    ));

    let parse_symbol = alt((
        map_res( many1(parse_symbol_character()), join),
        map_res(parse_special_symbols, str_to_string)
    ));
    let parse_symbol_element = map_res(parse_symbol, make_symbol_element);

    parse_symbol_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_on_simple_value() {
        assert_eq!(Ok(("", SymbolElement {value: "test".to_string()})), parse_symbol_element("test"));
    }

    #[test]
    fn able_to_parse_all_fine_symbols() {
        let example = "test1-_^v=+?<>./&*%$@!~";
        assert_eq!(Ok(("", SymbolElement {value: String::from(example)})), parse_symbol_element(example));
    }

    #[test]
    fn able_to_parse_all_fine_escaped_symbols() {
        let text = r##"test\"\,\`\ \(\)\:\\\{\}"##;
        let expected = r##"test",` ():\{}"##;

        assert_eq!(Ok(("", SymbolElement {value: String::from(expected)})), parse_symbol_element(text));
    }

    #[test]
    fn allows_numbers_not_at_the_first_position() {
        assert_eq!(Ok(("", SymbolElement {value: String::from("test1")})), parse_symbol_element("test1"));
        assert_eq!(Ok(("", SymbolElement {value: String::from("1test")})), parse_symbol_element("1test"));
    }

    #[test]
    fn parses_special_symbols() {
        assert_eq!(Ok(("", SymbolElement {value: String::from("#opt")})), parse_symbol_element("#opt"));
        assert_eq!(Ok(("", SymbolElement {value: String::from("#rest")})), parse_symbol_element("#rest"));
        assert_eq!(Ok(("", SymbolElement {value: String::from("#keys")})), parse_symbol_element("#keys"));
    }

    #[test]
    fn does_not_parse_invalid_special_symbols() {
        assert_eq!(Err(nom::Err::Error(("#tt", nom::error::ErrorKind::Tag))), parse_symbol_element("#tt"));
    }
}
