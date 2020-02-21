use nom::{
    character::complete::{
        space0
    },
    bytes::complete::tag,
    combinator::{
        map_res
    },
    multi::{
        many0
    },
};

use crate::parser::{Element, parse_element};
use nom::sequence::{preceded, terminated};

#[derive(Debug)]
pub struct SExpressionElement {
    values: Vec<Element>,
}

impl SExpressionElement {
    pub fn new(values: Vec<Element>) -> SExpressionElement {
        SExpressionElement {
            values
        }
    }

    pub fn get_values(self) -> Vec<Element> {
        self.values
    }
}

impl PartialEq for SExpressionElement {
    fn eq(&self, other: &Self) -> bool {
        if self.values.len() != other.values.len() {
            return false;
        }

        let len = self.values.len();

        for i in 0..len {
            if self.values[i] != other.values[i] {
                return false
            }
        }

        return true;
    }
}

fn make_s_expression_element(values: Vec<Element>) -> Result<SExpressionElement, String> {
    Ok(SExpressionElement::new(values))
}

pub fn parse_s_expression_element(s: &str) -> Result<(&str, SExpressionElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse_expressions = many0(preceded(space0, parse_element));

    let opening_brace = terminated(tag("("), space0);
    let closing_brace = preceded(space0, tag(")"));

    let parse_s_expression = preceded(
        opening_brace,
        terminated(
            parse_expressions,
            closing_brace
        )
    );

    let parse_s_expression_element = map_res(parse_s_expression, make_s_expression_element);

    parse_s_expression_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::integer_element::IntegerElement;
    use crate::parser::float_element::FloatElement;
    use crate::parser::string_element::StringElement;
    use crate::parser::boolean_element::BooleanElement;
    use crate::parser::keyword_element::KeywordElement;
    use crate::parser::symbol_element::SymbolElement;

    fn assert_s_expression_parsed_correctly(expected: Vec<Element>, code: &str) {
        assert_eq!(expected, parse_s_expression_element(code).ok().unwrap().1.values);
    }

    #[test]
    fn test_simple_s_expression() {
        assert_s_expression_parsed_correctly(
            vec!(
                Element::Symbol(SymbolElement::new(String::from("test"))),
                Element::Integer(IntegerElement::new(1)),
                Element::Float(FloatElement::new(1.0)),
                Element::String(StringElement::new(String::from("test"))),
                Element::Boolean(BooleanElement::new(true)),
                Element::Boolean(BooleanElement::new(false)),
                Element::Keyword(KeywordElement::new(String::from("keyword"))),
            ),
            r#"(test 1 1.0 "test" #t #f :keyword)"#
        );
    }

    #[test]
    fn test_spaces_are_processed_correctly() {
        assert_s_expression_parsed_correctly(
            vec!(
                Element::Symbol(SymbolElement::new(String::from("test"))),
            ),
            r#"(test)"#
        );
        assert_s_expression_parsed_correctly(
            vec!(
                Element::Symbol(SymbolElement::new(String::from("test"))),
            ),
            r#"( test)"#
        );
        assert_s_expression_parsed_correctly(
            vec!(
                Element::Symbol(SymbolElement::new(String::from("test"))),
            ),
            r#"(test )"#
        );
        assert_s_expression_parsed_correctly(
            vec!(
                Element::Symbol(SymbolElement::new(String::from("test"))),
            ),
            r#"( test )"#
        );
    }

    #[test]
    fn test_nested_s_expressions_are_processed() {
        assert_s_expression_parsed_correctly(
            vec!(
                Element::Symbol(SymbolElement::new(String::from("test"))),
                Element::Keyword(KeywordElement::new(String::from("list"))),
                Element::SExpression(SExpressionElement::new(vec!(
                    Element::Symbol(SymbolElement::new(String::from("b"))),
                    Element::Integer(IntegerElement::new(1)),
                    Element::Integer(IntegerElement::new(2)),
                )))
            ),
            r#"(test :list (b 1 2))"#
        );
        assert_s_expression_parsed_correctly(
            vec!(
                Element::Symbol(SymbolElement::new(String::from("test"))),
                Element::SExpression(SExpressionElement::new(vec!(
                    Element::Symbol(SymbolElement::new(String::from("test"))),
                    Element::SExpression(SExpressionElement::new(vec!(
                        Element::Symbol(SymbolElement::new(String::from("b"))),
                        Element::Integer(IntegerElement::new(1)),
                        Element::Integer(IntegerElement::new(2)),
                    )))
                )))
            ),
            r#"(test (test (b 1 2)))"#
        );
    }
}
