use nom::{
    bytes::complete::tag,
    branch::alt,
    combinator::{
        map_res
    },
};

#[derive(Debug)]
pub struct BooleanElement {
    value: bool,
}

impl BooleanElement {
    pub fn new(value: bool) -> BooleanElement {
        BooleanElement {
            value
        }
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

fn make_boolean_element(s: &str) -> Result<BooleanElement, String> {
    if s == "#t" {
        return Ok(BooleanElement::new(true));
    } else if s == "#f" {
        return Ok(BooleanElement::new(false));
    } else {
        unreachable!();
    }
}

pub fn parse_boolean_element(s: &str) -> Result<(&str, BooleanElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse_boolean = alt((tag("#t"), tag("#f")));

    let parse_element = map_res(parse_boolean, make_boolean_element);

    parse_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_true_correctly() {
        assert_eq!(Ok(("", BooleanElement{ value: true})), parse_boolean_element("#t"))
    }

    #[test]
    fn test_parses_false_correctly() {
        assert_eq!(Ok(("", BooleanElement{ value: false})), parse_boolean_element("#f"))
    }
}
