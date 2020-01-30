use nom::{
    combinator::{
        recognize,
        map_res
    },
    multi::many1,
};
use crate::parser::lib::parse_symbol_character;

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

fn make_symbol_element(value: &str) -> Result<SymbolElement, String> {
    Ok(SymbolElement::new(value.to_string()))
}

pub fn parse_symbol_element(s: &str) -> Result<(&str, SymbolElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse_symbol = recognize(many1(parse_symbol_character()));
    let parse_symbol_element = map_res(parse_symbol, make_symbol_element);

    parse_symbol_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_works_on_simple_value() {
        assert_eq!(Ok(("", SymbolElement {value: "test".to_string()})), parse_symbol_element("test"));
    }

    #[test]
    fn test_able_to_parse_all_fine_symbols() {
        let example = "test1-_^v=+?<>./&*%$@!~{}";
        assert_eq!(Ok(("", SymbolElement {value: String::from(example)})), parse_symbol_element(example));
    }

    //todo: test for escaped
    #[test]
    fn test_able_to_parse_all_fine_escaped_symbols() {
        let example = r##""test1\#\,\`\ \(\)\:\\"##;
        assert_eq!(Ok(("", SymbolElement {value: String::from(example)})), parse_symbol_element(example));
    }

    #[test]
    fn test_allows_numbers_not_at_the_first_position() {
        assert_eq!(Ok(("", SymbolElement {value: String::from("test1")})), parse_symbol_element("test1"));
        assert_eq!(Ok(("", SymbolElement {value: String::from("1test")})), parse_symbol_element("1test"));
    }
}
