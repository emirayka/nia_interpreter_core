use nom::{
    bytes::complete::{
        tag
    },
    sequence::{
        preceded
    },
    combinator::{
        map_res
    },
    multi::many1,
};

use crate::parser::lib::parse_keyword_character;

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

fn join(chars: Vec<char>) -> Result<String, String> {
    Ok(chars.iter().collect())
}

fn make_keyword_element(value: String) -> Result<KeywordElement, String> {
    Ok(KeywordElement::new(value))
}

pub fn parse_keyword_element(s: &str) -> Result<(&str, KeywordElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse_keyword = map_res(
        preceded(tag(":"), many1(parse_keyword_character())),
        join);
    let parse_keyword_element = map_res(parse_keyword, make_keyword_element);

    parse_keyword_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;

    macro_rules! assert_keyword_parsing_is_ok {
        ($code:expr, $rest:expr) => {
            assert_eq!(
                Ok(($rest, KeywordElement {value: String::from(&$code[':'.len_utf8()..])})),
                 parse_keyword_element($code)
            );
        };
        ($code:expr) => {
            assert_keyword_parsing_is_ok!($code, "");
        };
    }

    #[test]
    fn test_works_on_simple_value() {
        assert_keyword_parsing_is_ok!(":test");
        assert_eq!(Err(nom::Err::Error(("test", ErrorKind::Tag))), parse_keyword_element("test"));
    }

    #[test]
    fn test_allows_numbers() {
        assert_keyword_parsing_is_ok!(":test1");
        assert_keyword_parsing_is_ok!(":1test");
    }

    #[test]
    fn test_able_to_parse_all_fine_symbols() {
        let example= ":::test1-_^v=+?<>./&*%$@!~{}";
        assert_keyword_parsing_is_ok!(example);
    }

    #[test]
    fn test_able_to_parse_all_fine_escaped_symbols() {
        let example = r##":::test1\"\#\,\`\ \(\)\\"##;
        let expected = r##"::test1"#,` ()\"##;

        assert_eq!(
            Ok(("", KeywordElement {value: String::from(expected)})),
            parse_keyword_element(example)
        );
    }
}
