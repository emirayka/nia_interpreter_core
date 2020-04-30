use nom::{
    named,
    preceded,
    tag,
    map_res,
};

use crate::parser::s_expression_element;
use crate::parser::SExpressionElement;

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

named!(parse_body(&str) -> SExpressionElement, preceded!(
    tag!("#"),
    s_expression_element::parse
));

named!(pub parse(&str) -> ShortLambdaElement, map_res!(
    parse_body,
    make_short_lambda_element
));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::integer_element::IntegerElement;
    use crate::parser::element::Element;

    fn assert_short_lambda_element_parsed_correctly(expected: ShortLambdaElement, code: &str) {
        assert_eq!(Ok(("", expected)), parse(code))
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
