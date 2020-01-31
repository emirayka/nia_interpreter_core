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

    pub fn get_values(&self) -> &Vec<Element> {
        &self.values
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

    #[test]
    fn test_simple_s_expression() {
        println!("{:?}", parse_s_expression_element(r#"(test 1 1.0 "test" #t #f :keyword)"#));
    }

    #[test]
    fn test_spaces_are_processed_correctly() {
        println!("{:?}", parse_s_expression_element(r#"(test)"#));
        println!("{:?}", parse_s_expression_element(r#"( test)"#));
        println!("{:?}", parse_s_expression_element(r#"(test )"#));
        println!("{:?}", parse_s_expression_element(r#"( test )"#));
    }

    #[test]
    fn test_nested_s_expressions_are_processed() {
        println!("{:?}", parse_s_expression_element(r#"(test :list (b 1 2))"#));
        println!("{:?}", parse_s_expression_element(r#"(test (test (b 1 2)))"#));
    }
}
