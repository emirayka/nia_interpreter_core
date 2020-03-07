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

    pub fn get_value_ref(&self) -> &Element {
        self.value.as_ref()
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
    use crate::parser::symbol_element::SymbolElement;
    use crate::parser::integer_element::IntegerElement;
    use crate::parser::float_element::FloatElement;
    use crate::parser::boolean_element::BooleanElement;
    use crate::parser::string_element::StringElement;
    use crate::parser::s_expression_element::SExpressionElement;

    fn assert_prefixed_element_parsed_correctly(
        expected_prefix: Prefix,
        expected_element: Element,
        code: &str
    ) {
        assert_eq!(
            Ok(("", PrefixElement::new(expected_prefix, expected_element))),
            parse_prefixed_element(code)
        )
    }

    fn assert_prefixed_element_parsed_correctly_2(
        expected_prefix_1: Prefix,
        expected_prefix_2: Prefix,
        expected_element: Element,
        code: &str
    ) {
        assert_eq!(
            Ok(("", PrefixElement::new(
                expected_prefix_1,
                Element::Prefix(PrefixElement::new(
                    expected_prefix_2,
                    expected_element
                ))))
            ),
            parse_prefixed_element(code)
        )
    }

    fn assert_prefix_works(expected_prefix: Prefix, expected_prefix_code: &str) {
        assert_prefixed_element_parsed_correctly(
            expected_prefix,
            Element::Symbol(SymbolElement::new(String::from("a"))),
            &format!("{}{}", expected_prefix_code, "a")
        );

        assert_prefixed_element_parsed_correctly(
            expected_prefix,
            Element::Integer(IntegerElement::new(1)),
            &format!("{}{}", expected_prefix_code, "1")
        );

        assert_prefixed_element_parsed_correctly(
            expected_prefix,
            Element::Float(FloatElement::new(1.0)),
            &format!("{}{}", expected_prefix_code, "1.0")
        );

        assert_prefixed_element_parsed_correctly(
            expected_prefix,
            Element::Boolean(BooleanElement::new(true)),
            &format!("{}{}", expected_prefix_code, "#t")
        );

        assert_prefixed_element_parsed_correctly(
            expected_prefix,
            Element::Boolean(BooleanElement::new(false)),
            &format!("{}{}", expected_prefix_code, "#f")
        );

        assert_prefixed_element_parsed_correctly(
            expected_prefix,
            Element::String(StringElement::new(String::from("tt"))),
            &format!("{}{}", expected_prefix_code, "\"tt\"")
        );

        assert_prefixed_element_parsed_correctly(
            expected_prefix,
            Element::SExpression(SExpressionElement::new(vec!(
                Element::Symbol(SymbolElement::new(String::from("b"))),
                Element::Integer(IntegerElement::new(1)),
                Element::Integer(IntegerElement::new(2)),
            ))),
            &format!("{}{}", expected_prefix_code, "(b 1 2)")
        );
    }

    fn assert_prefix_prefix_works(
        expected_prefix_1: Prefix,
        expected_prefix_2: Prefix,
        expected_prefix_code: &str
    ) {
        assert_prefixed_element_parsed_correctly_2(
            expected_prefix_1,
            expected_prefix_2,
            Element::Symbol(SymbolElement::new(String::from("a"))),
            &format!("{}{}", expected_prefix_code, "a")
        );

        assert_prefixed_element_parsed_correctly_2(
            expected_prefix_1,
            expected_prefix_2,
            Element::Integer(IntegerElement::new(1)),
            &format!("{}{}", expected_prefix_code, "1")
        );

        assert_prefixed_element_parsed_correctly_2(
            expected_prefix_1,
            expected_prefix_2,
            Element::Float(FloatElement::new(1.0)),
            &format!("{}{}", expected_prefix_code, "1.0")
        );

        assert_prefixed_element_parsed_correctly_2(
            expected_prefix_1,
            expected_prefix_2,
            Element::Boolean(BooleanElement::new(true)),
            &format!("{}{}", expected_prefix_code, "#t")
        );

        assert_prefixed_element_parsed_correctly_2(
            expected_prefix_1,
            expected_prefix_2,
            Element::Boolean(BooleanElement::new(false)),
            &format!("{}{}", expected_prefix_code, "#f")
        );

        assert_prefixed_element_parsed_correctly_2(
            expected_prefix_1,
            expected_prefix_2,
            Element::String(StringElement::new(String::from("tt"))),
            &format!("{}{}", expected_prefix_code, "\"tt\"")
        );

        assert_prefixed_element_parsed_correctly_2(
            expected_prefix_1,
            expected_prefix_2,
            Element::SExpression(SExpressionElement::new(vec!(
                Element::Symbol(SymbolElement::new(String::from("b"))),
                Element::Integer(IntegerElement::new(1)),
                Element::Integer(IntegerElement::new(2)),
            ))),
            &format!("{}{}", expected_prefix_code, "(b 1 2)")
        );
    }

    #[test]
    fn simple_prefixed_values() {
        assert_prefix_works(Prefix::Quote, "'");
        assert_prefix_works(Prefix::Comma, ",");
        assert_prefix_works(Prefix::CommaDog, ",@");
        assert_prefix_works(Prefix::GraveAccent, "`");
    }

    #[test]
    fn already_prefixed_prefixed_values() {
        let prefixes = vec!(
            (Prefix::Quote, "'"),
            (Prefix::Comma, ","),
            (Prefix::CommaDog, ",@"),
            (Prefix::GraveAccent, "`"),
        );

        for prefix_1 in &prefixes {
            for prefix_2 in &prefixes {
                assert_prefix_prefix_works(
                    prefix_1.0,
                    prefix_2.0,
                    &format!("{}{}", prefix_1.1, prefix_1.1)
                );
            }
        }
    }
}

