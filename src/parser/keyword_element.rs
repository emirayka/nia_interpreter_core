use nom::{
    character::complete::{
        alpha1,
        alphanumeric0
    },
    bytes::complete::{
        tag
    },
    sequence::{
        tuple,
        preceded
    },
    combinator::{
        recognize,
        map_res
    }
};

#[derive(Debug)]
pub struct KeywordElement {
    value: String,
}

impl KeywordElement {
    pub fn new(value: String) -> KeywordElement {
        KeywordElement {
            value
        }
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}

impl PartialEq for KeywordElement {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

fn make_keyword_element(value: &str) -> Result<KeywordElement, String> {
    Ok(KeywordElement::new(value.to_string()))
}

pub fn parse_keyword_element(s: &str) -> Result<(&str, KeywordElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    let one_colon = tag(":");
    let parse_keyword = preceded(one_colon, recognize(tuple((alpha1, alphanumeric0))));
    let parse_keyword_element = map_res(parse_keyword, make_keyword_element);

    parse_keyword_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn test_works_on_simple_value() {
        assert_eq!(Ok(("", KeywordElement {value: "test".to_string()})), parse_keyword_element(":test"));
        assert_eq!(Err(nom::Err::Error(("test", ErrorKind::Tag))), parse_keyword_element("test"));
    }

    #[test]
    fn test_allows_numbers_not_at_the_first_position() {
        assert_eq!(Ok(("", KeywordElement {value: "test1".to_string()})), parse_keyword_element(":test1"));
        assert_eq!(Err(nom::Err::Error(("1test", ErrorKind::Alpha))), parse_keyword_element(":1test"));
    }
}
