use nom::{
    character::complete::digit1,
    bytes::complete::tag,
    branch::alt,
    sequence::pair,
    combinator::{
        recognize,
        opt,
        map_res
    },
    error::ErrorKind
};

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

fn make_integer_element(value: i64) -> Result<IntegerElement, String> {
    Ok(IntegerElement::new(value))
}

pub fn parse_integer_element(s: &str) -> Result<(&str, IntegerElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse_plus_sign = tag::<_, _, (&str, ErrorKind)>("+");
    let parse_minus_sign = tag::<_, _, (&str, ErrorKind)>("-");
    let parse_sign = alt((parse_plus_sign, parse_minus_sign));

    let parse_integer = recognize(pair(opt(parse_sign), digit1));

    let parse_i64 = map_res(parse_integer, |s: &str| s.parse::<i64>());
    let parse_integer_element = map_res(parse_i64, make_integer_element);

    parse_integer_element(s)
}

//named!(parse_i64<&str, i64>, map_res!(parse_integer, |s: &str| s.parse::<i64>()));
//named!(parse_integer_element<&str, IntegerElement>, map_res!(parse_i64, make_integer_element));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_value() {
        assert_eq!(Ok(("", IntegerElement{value: 20})), parse_integer_element("20"));
    }

    #[test]
    fn test_signed_positive_value() {
        assert_eq!(Ok(("", IntegerElement{value: 20})), parse_integer_element("+20"));
    }

    #[test]
    fn test_signed_negative_value() {
        assert_eq!(Ok(("", IntegerElement{value: -20})), parse_integer_element("-20"));
    }
}
