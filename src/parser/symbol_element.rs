use nom::{
    character::complete::{
        alpha1,
        alphanumeric0
    },
    sequence::pair,
    combinator::{
        recognize,
        map_res
    }
};

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
    let parse_symbol = recognize(pair(alpha1, alphanumeric0));
    let parse_symbol_element = map_res(parse_symbol, make_symbol_element);

    parse_symbol_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn test_works_on_simple_value() {
        assert_eq!(Ok(("", SymbolElement {value: "test".to_string()})), parse_symbol_element("test"));
    }

    #[test]
    fn test_allows_numbers_not_at_the_first_position() {
        assert_eq!(Ok(("", SymbolElement {value: "test1".to_string()})), parse_symbol_element("test1"));
        assert_eq!(Err(nom::Err::Error(("1test", ErrorKind::Alpha))), parse_symbol_element("1test"));
    }
}
