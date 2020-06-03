use nom::alt;
use nom::character::complete::digit1;
use nom::character::complete::hex_digit1;
use nom::complete;
use nom::map_res;
use nom::named;
use nom::opt;
use nom::pair;
use nom::recognize;
use nom::tag;
use nom::tuple;

use crate::interpreter::parser::ParseError;

#[derive(Debug, Clone)]
pub struct IntegerElement {
    value: i64,
}

impl IntegerElement {
    pub fn new(value: i64) -> IntegerElement {
        IntegerElement { value }
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

impl Eq for IntegerElement {}

fn make_integer_element(value: i64) -> Result<IntegerElement, ParseError> {
    Ok(IntegerElement::new(value))
}

named!(parse_hex_prefix(&str) -> &str, alt!(tag!("0x")));
named!(parse_sign(&str) -> &str, alt!(tag!("+") | tag!("-")));

named!(parse_positive_hex(&str) -> &str, recognize!(
    complete!(tuple!(tag!("+"), parse_hex_prefix, hex_digit1)))
);
named!(parse_negative_hex(&str) -> &str, recognize!(
    complete!(tuple!(tag!("-"), parse_hex_prefix, hex_digit1)))
);
named!(parse_hex(&str) -> &str, recognize!(
    complete!(pair!(parse_hex_prefix, hex_digit1)))
);

named!(parse_hex_i64(&str) -> i64, alt!(
    map_res!(parse_hex, |s: &str| i64::from_str_radix(&s[('0'.len_utf8() + 'x'.len_utf8())..], 16)) |
    map_res!(parse_positive_hex, |s: &str| i64::from_str_radix(&s[('+'.len_utf8() + '0'.len_utf8() + 'x'.len_utf8())..], 16)) |
    map_res!(parse_negative_hex, |s: &str| i64::from_str_radix(&s[('-'.len_utf8() + '0'.len_utf8() + 'x'.len_utf8())..], 16).map(|v: i64| -v))
));

named!(parse_decimal(&str) -> &str, recognize!(pair!(opt!(parse_sign), digit1)));
named!(parse_decimal_i64(&str) -> i64, map_res!(parse_decimal, |s: &str| s.parse::<i64>()));

named!(parse_i64<&str, i64>, alt!(parse_hex_i64 | parse_decimal_i64 ));

named!(pub parse<&str, IntegerElement>, map_res!(parse_i64, make_integer_element));

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn parses_zero() {
        nia_assert_equal(Ok(("", IntegerElement { value: 0 })), parse("0"));
    }

    #[test]
    fn parses_one() {
        nia_assert_equal(Ok(("", IntegerElement { value: 1 })), parse("1"));
    }

    #[test]
    fn parses_unsigned_value() {
        nia_assert_equal(Ok(("", IntegerElement { value: 20 })), parse("20"));
    }

    #[test]
    fn parses_signed_positive_value() {
        nia_assert_equal(Ok(("", IntegerElement { value: 20 })), parse("+20"));
    }

    #[test]
    fn parses_signed_negative_value() {
        nia_assert_equal(Ok(("", IntegerElement { value: -20 })), parse("-20"));
    }

    #[test]
    fn parses_unsigned_hex_value() {
        nia_assert_equal(Ok(("", IntegerElement { value: 47 })), parse("0x2f"));
    }

    #[test]
    fn parses_signed_positive_hex_value() {
        nia_assert_equal(
            Ok(("", IntegerElement { value: 47 })),
            parse("+0x2f"),
        );
    }

    #[test]
    fn parses_signed_negative_hex_value() {
        nia_assert_equal(
            Ok(("", IntegerElement { value: -47 })),
            parse("-0x2f"),
        );
    }

    #[test]
    fn returns_remaining_input() {
        nia_assert_equal(
            Ok((" kek", IntegerElement { value: -20 })),
            parse("-20 kek"),
        );
    }

    #[test]
    fn returns_error() {
        nia_assert(parse("-").is_err());
    }
}
