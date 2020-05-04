use nom::{
    alt, character::complete::multispace1, combinator::all_consuming, map_res, named, peek, tag,
    terminated,
};

use crate::parser::ParseError;

#[derive(Debug)]
pub struct BooleanElement {
    value: bool,
}

impl BooleanElement {
    pub fn new(value: bool) -> BooleanElement {
        BooleanElement { value }
    }

    pub fn get_value(&self) -> bool {
        self.value
    }
}

impl PartialEq for BooleanElement {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for BooleanElement {}

fn make_boolean_true(s: &str) -> Result<BooleanElement, ParseError> {
    Ok(BooleanElement::new(true))
}

fn make_boolean_false(s: &str) -> Result<BooleanElement, ParseError> {
    Ok(BooleanElement::new(false))
}

named!(parse_boolean_true_literal(&str) -> &str, tag!("#t"));
named!(parse_boolean_false_literal(&str) -> &str, tag!("#f"));

named!(parse_boolean_true(&str) -> BooleanElement, map_res!(tag!("#t"), make_boolean_true));
named!(parse_boolean_false(&str) -> BooleanElement, map_res!(tag!("#f"), make_boolean_false));

named!(
    pub parse(&str) -> BooleanElement,
    alt!(parse_boolean_true | parse_boolean_false)
);

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn parses_true_correctly() {
        nia_assert_equal(Ok(("", BooleanElement { value: true })), parse("#t"))
    }

    #[test]
    fn parses_false_correctly() {
        nia_assert_equal(Ok(("", BooleanElement { value: false })), parse("#f"))
    }

    #[test]
    fn returns_correct_remaining_input() {
        nia_assert_equal(Ok((" #f", BooleanElement { value: false })), parse("#f #f"))
    }

    #[test]
    fn returns_correct_errors() {
        nia_assert(parse("#kek").is_err());
    }
}
