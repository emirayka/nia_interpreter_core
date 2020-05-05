use nom::{many1, map_res, named, preceded, tag};

use crate::parser::lib::parse_keyword_character;
use crate::parser::ParseError;

#[derive(Debug, Clone)]
pub struct KeywordElement {
    value: String,
}

impl KeywordElement {
    pub fn new(value: String) -> KeywordElement {
        KeywordElement { value }
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

impl Eq for KeywordElement {}

fn join(chars: Vec<char>) -> Result<String, ParseError> {
    Ok(chars.iter().collect())
}

fn make_keyword_element(value: String) -> Result<KeywordElement, ParseError> {
    Ok(KeywordElement::new(value))
}

named!(parse_colon(&str) -> &str, tag!(":"));

named!(parse_keyword(&str) -> String, map_res!(
    preceded!(
        parse_colon,
        many1!(parse_keyword_character)
    ),
    join
));

named!(pub parse(&str) -> KeywordElement, map_res!(
    parse_keyword,
    make_keyword_element
));

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use nom::error::ErrorKind;

    macro_rules! assert_keyword_parsing_is_ok {
        ($code:expr, $rest:expr) => {
            nia_assert_equal(
                Ok((
                    $rest,
                    KeywordElement {
                        value: String::from(&$code[':'.len_utf8()..]),
                    },
                )),
                parse($code),
            );
        };
        ($code:expr) => {
            assert_keyword_parsing_is_ok!($code, "");
        };
    }

    #[test]
    fn works_on_simple_value() {
        assert_keyword_parsing_is_ok!(":test");
        nia_assert_equal(
            Err(nom::Err::Error(("test", ErrorKind::Tag))),
            parse("test"),
        );
    }

    #[test]
    fn allows_numbers() {
        assert_keyword_parsing_is_ok!(":test1");
        assert_keyword_parsing_is_ok!(":1test");
    }

    #[test]
    fn able_to_parse_all_fine_symbols() {
        let example = ":::test1-_^v=+?<>./&*%$@!~";
        assert_keyword_parsing_is_ok!(example);
    }

    #[test]
    fn able_to_parse_all_fine_escaped_symbols() {
        let example = r##":::test1\"\,\`\ \(\)\\\{\}"##;
        let expected = r##"::test1",` ()\{}"##;

        nia_assert_equal(
            Ok((
                "",
                KeywordElement {
                    value: String::from(expected),
                },
            )),
            parse(example),
        );
    }
}
