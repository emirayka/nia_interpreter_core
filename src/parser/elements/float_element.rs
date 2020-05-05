use nom::{
    alt, character::complete::digit1, complete, map_res, named, opt, pair,
    recognize, tag, tuple,
};

use crate::parser::ParseError;

#[derive(Debug)]
pub struct FloatElement {
    value: f64,
}

impl FloatElement {
    pub fn new(value: f64) -> FloatElement {
        FloatElement { value }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

impl PartialEq for FloatElement {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for FloatElement {}

fn make_float_element(value: f64) -> Result<FloatElement, ParseError> {
    Ok(FloatElement::new(value))
}

named!(parse_sign(&str) -> &str, alt!(tag!("+") | tag!("-")));
named!(parse_dot(&str) -> &str, tag!("."));
named!(parse_exponent_character(&str) -> &str, alt!(tag!("e") | tag!("E")));

named!(parse_integer_part(&str) -> &str, recognize!(pair!(opt!(parse_sign), digit1)));
named!(parse_exponent(&str) -> Option<(&str, &str)>, opt!(
    complete!(pair!(parse_exponent_character, parse_integer_part))
));

named!(parse_float(&str) -> &str, recognize!(complete!(tuple!(
    parse_integer_part,
    parse_dot,
    digit1,
    parse_exponent
))));

named!(parse_f64<&str, f64>, map_res!(parse_float, |s: &str| s.parse::<f64>()));
named!(pub parse<&str, FloatElement>, map_res!(parse_f64, make_float_element));

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use std::str::FromStr;

    macro_rules! make_float_assertion {
        ($str:expr) => {
            nia_assert_equal(
                Ok((
                    "",
                    FloatElement {
                        value: f64::from_str($str).unwrap(),
                    },
                )),
                parse($str),
            );
        };
    }

    macro_rules! make_failed_float_test {
        ($name:ident, $str:expr) => {
            #[test]
            #[should_panic]
            fn $name() {
                make_float_assertion!($str);
            }
        };
    }

    #[test]
    fn simple_values() {
        make_float_assertion!("77.77");
        make_float_assertion!("-77.77");
        make_float_assertion!("+77.77");

        make_float_assertion!("77.77E77");
        make_float_assertion!("-77.77E-77");
        make_float_assertion!("+77.77E+77");
    }

    make_failed_float_test!(int_is_not_float_1, "77");
    make_failed_float_test!(int_is_not_float_2, "+77");
    make_failed_float_test!(int_is_not_float_3, "-77");
    make_failed_float_test!(int_is_not_float_even_with_exponent_1, "77e77");
    make_failed_float_test!(int_is_not_float_even_with_exponent_2, "-77e-77");
    make_failed_float_test!(int_is_not_float_even_with_exponent_3, "+77e+77");
}
