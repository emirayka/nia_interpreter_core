use nom::{
    named,
    alt,
    tag,
    recognize,
    pair,
    opt,
    map_res,
    character::complete::digit1,
};
use crate::parser::ParseError;

#[derive(Debug)]
pub struct IntegerElement {
    value: i64,
}

impl IntegerElement {
    pub fn new(value: i64) -> IntegerElement {
        IntegerElement {
            value
        }
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }
}

impl PartialEq for IntegerElement {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

fn make_integer_element(value: i64) -> Result<IntegerElement, ParseError> {
    Ok(IntegerElement::new(value))
}

named!(parse_sign(&str) -> &str, alt!(tag!("+") | tag!("-")));
named!(parse_integer(&str) -> &str, recognize!(pair!(opt!(parse_sign), digit1)));

named!(parse_i64<&str, i64>, map_res!(parse_integer, |s: &str| s.parse::<i64>()));
named!(pub parse<&str, IntegerElement>, map_res!(parse_i64, make_integer_element));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_unsigned_value() {
        assert_eq!(Ok(("", IntegerElement{value: 20})), parse("20"));
    }

    #[test]
    fn parses_signed_positive_value() {
        assert_eq!(Ok(("", IntegerElement{value: 20})), parse("+20"));
    }

    #[test]
    fn parses_signed_negative_value() {
        assert_eq!(Ok(("", IntegerElement{value: -20})), parse("-20"));
    }

    #[test]
    fn returns_remaining_input() {
        assert_eq!(Ok((" kek", IntegerElement{value: -20})), parse("-20 kek"));
    }

    #[test]
    fn returns_error() {
        assert!(parse("-").is_err());
    }
}
