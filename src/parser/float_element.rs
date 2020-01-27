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
};
use nom::sequence::tuple;
use nom::combinator::complete;

#[derive(Debug)]
pub struct FloatElement {
    value: f64,
}

impl FloatElement {
    pub fn new(value: f64) -> FloatElement {
        FloatElement {
            value
        }
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

fn make_float_element(value: f64) -> Result<FloatElement, String> {
    Ok(FloatElement::new(value))
}

pub fn parse_float_element(s: &str) -> Result<(&str, FloatElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    // todo: rewrite it somehow
    let parse_float = recognize(
        tuple((
                recognize(pair(opt(alt((tag("-"), tag("+")))),digit1)),
                tag("."),
                digit1,
                opt(complete(pair(
                    alt((tag("e"), tag("E"))),
                    recognize(pair(opt(alt((tag("-"), tag("+")))),digit1)),
                )))
            ))
    );

    let parse_f64 = map_res(parse_float, |s: &str| s.parse::<f64>());
    let parse_float_element = map_res(parse_f64, make_float_element);

    parse_float_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    macro_rules! make_float_assertion {
        ($str:expr) => {
            assert_eq!(Ok(("", FloatElement {value: f64::from_str($str).unwrap()})), parse_float_element($str));
        }
    }

    macro_rules! make_failed_float_test {
        ($name:ident, $str:expr) => {
            #[test]
            #[should_panic]
            fn $name() {
                make_float_assertion!($str);
            }
        }
    }

    #[test]
    fn test_simple_values() {
        make_float_assertion!("77.77");
        make_float_assertion!("-77.77");
        make_float_assertion!("+77.77");

        make_float_assertion!("77.77E77");
        make_float_assertion!("-77.77E-77");
        make_float_assertion!("+77.77E+77");
    }

    make_failed_float_test!(test_int_is_not_float_1, "77");
    make_failed_float_test!(test_int_is_not_float_2, "+77");
    make_failed_float_test!(test_int_is_not_float_3, "-77");
    make_failed_float_test!(test_int_is_not_float_even_with_exponent_1, "77e77");
    make_failed_float_test!(test_int_is_not_float_even_with_exponent_2, "-77e-77");
    make_failed_float_test!(test_int_is_not_float_even_with_exponent_3, "+77e+77");
}
