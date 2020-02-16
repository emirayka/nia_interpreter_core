use nom::{
    bytes::complete::tag,
    branch::alt,
    sequence::pair,
    combinator::{
        recognize,
        opt,
        map_res
    },
};

use crate::parser::{Element, parse_element};

#[derive(Debug, Clone, Copy)]
pub enum Prefix {
    CommaDog,
    Comma,
    Quote,
    GraveAccent
}

fn make_prefix(s: &str) -> Result<Prefix, String> {
    let prefix = if s == ",@" {
        Prefix::CommaDog
    } else if s == "," {
        Prefix::Comma
    } else if s == "'" {
        Prefix::Quote
    } else if s == "`" {
        Prefix::GraveAccent
    } else {
        unreachable!()
    };

    Ok(prefix)
}

#[derive(Debug)]
pub struct PrefixElement {
    value: Box<Element>,
    prefix: Prefix
}

impl PrefixElement {
    pub fn new(prefix: Prefix, value: Element) -> PrefixElement {
        PrefixElement {
            value: Box::new(value),
            prefix
        }
    }

    pub fn get_prefix(&self) -> Prefix {
        self.prefix
    }

    pub fn get_value(self) -> Element {
        *self.value
    }
}

impl PartialEq for PrefixElement {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

fn make_prefixed_element(pair: (Prefix, Element)) -> Result<PrefixElement, ()> {
    Ok(PrefixElement::new(pair.0, pair.1))
}

pub fn parse_prefixed_element(s: &str) -> Result<(&str, PrefixElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse_prefix = alt(
        (
            tag("`"),
            tag("'"),
            recognize(pair(tag(","), opt(tag("@"))))
        )
    );

    let parse_prefix = map_res(parse_prefix, make_prefix);

    let parse_prefixed = pair(parse_prefix, parse_element);
    let parse_prefixed_element = map_res(parse_prefixed, make_prefixed_element);

    parse_prefixed_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_prefixed_values() {
        println!("{:?}", parse_prefixed_element("'a"));
        println!("{:?}", parse_prefixed_element("'1"));
        println!("{:?}", parse_prefixed_element("'1.0"));
        println!("{:?}", parse_prefixed_element("'#t"));
        println!("{:?}", parse_prefixed_element("'#f"));
        println!("{:?}", parse_prefixed_element("'\"tt\""));
        println!("{:?}", parse_prefixed_element("'(b 1 2)"));

        println!("{:?}", parse_prefixed_element(",a"));
        println!("{:?}", parse_prefixed_element(",1"));
        println!("{:?}", parse_prefixed_element(",1.0"));
        println!("{:?}", parse_prefixed_element(",#t"));
        println!("{:?}", parse_prefixed_element(",#f"));
        println!("{:?}", parse_prefixed_element(",\"tt\""));
        println!("{:?}", parse_prefixed_element(",(b 1 2)"));

        println!("{:?}", parse_prefixed_element(",@a"));
        println!("{:?}", parse_prefixed_element(",@1"));
        println!("{:?}", parse_prefixed_element(",@1.0"));
        println!("{:?}", parse_prefixed_element(",@#t"));
        println!("{:?}", parse_prefixed_element(",@#f"));
        println!("{:?}", parse_prefixed_element(",@\"tt\""));
        println!("{:?}", parse_prefixed_element(",@(b 1 2)"));

        println!("{:?}", parse_prefixed_element("`a"));
        println!("{:?}", parse_prefixed_element("`1"));
        println!("{:?}", parse_prefixed_element("`1.0"));
        println!("{:?}", parse_prefixed_element("`#t"));
        println!("{:?}", parse_prefixed_element("`#f"));
        println!("{:?}", parse_prefixed_element("`\"tt\""));
        println!("{:?}", parse_prefixed_element("`(b 1 2)"));
    }

    #[test]
    fn test_already_prefixed_prefixed_values() {
        println!("{:?}", parse_prefixed_element("''a"));
        println!("{:?}", parse_prefixed_element("''1"));
        println!("{:?}", parse_prefixed_element("''1.0"));
        println!("{:?}", parse_prefixed_element("''#t"));
        println!("{:?}", parse_prefixed_element("''#f"));
        println!("{:?}", parse_prefixed_element("''\"tt\""));
        println!("{:?}", parse_prefixed_element("''(b 1 2)"));
    }
}
