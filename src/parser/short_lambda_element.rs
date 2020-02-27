use nom::{
    bytes::complete::tag,
    combinator::{
        map_res
    },
    sequence::preceded
};

use crate::parser::{Element};
use crate::parser::s_expression_element::{SExpressionElement, parse_s_expression_element};

#[derive(Debug)]
pub struct ShortLambdaElement {
    s_expression: SExpressionElement,
}

impl ShortLambdaElement {
    pub fn new(s_expression: SExpressionElement) -> ShortLambdaElement {
        ShortLambdaElement {
            s_expression,
        }
    }

    pub fn get_value(self) -> SExpressionElement {
        self.s_expression
    }

    pub fn get_value_ref(&self) -> &SExpressionElement {
        &self.s_expression
    }
}

impl PartialEq for ShortLambdaElement {
    fn eq(&self, other: &Self) -> bool {
        return self.s_expression == other.s_expression
    }
}


fn make_short_lambda_element(s_expression: SExpressionElement) -> Result<ShortLambdaElement, String> {
    Ok(ShortLambdaElement::new(s_expression))
}

pub fn parse_short_lambda_element(
    s: &str
) -> Result<(&str, ShortLambdaElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse_short_lambda = preceded(tag("#"), parse_s_expression_element);
    let parse_short_lambda_element = map_res(
        parse_short_lambda,
        make_short_lambda_element
    );

    parse_short_lambda_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::integer_element::IntegerElement;

    fn assert_short_lambda_element_parsed_correctly(expected: ShortLambdaElement, code: &str) {
        assert_eq!(Ok(("", expected)), parse_short_lambda_element(code))
    }

    #[test]
    fn works_correctly() {
        assert_short_lambda_element_parsed_correctly(
            ShortLambdaElement::new(SExpressionElement::new(vec!(
                Element::Integer(IntegerElement::new(3))
            ))),
            "#(3)"
        )
    }
}
