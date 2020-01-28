pub mod integer_element;
pub mod float_element;
pub mod symbol_element;
pub mod keyword_element;
pub mod string_element;
pub mod boolean_element;
pub mod s_expression_element;
pub mod prefix_element;

use nom::{
    branch::alt,
    combinator::map_res,
    multi::many0,
    character::complete::{
        space0
    }
};
use nom::sequence::preceded;

#[derive(Debug)]
pub enum Element {
    Integer(integer_element::IntegerElement),
    Float(float_element::FloatElement),
    Boolean(boolean_element::BooleanElement),
    String(string_element::StringElement),
    Symbol(symbol_element::SymbolElement),
    Keyword(keyword_element::KeywordElement),
    SExpression(s_expression_element::SExpressionElement),
    Prefix(prefix_element::PrefixElement),
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        use Element::*;

        match (self, other) {
            (Integer(val1), Integer(val2)) => val1 == val2,
            (Float(val1), Float(val2)) => val1 == val2,
            (String(val1), String(val2)) => val1 == val2,
            (Symbol(val1), Symbol(val2)) => val1 == val2,
            (Keyword(val1), Keyword(val2)) => val1 == val2,
            (Boolean(val1), Boolean(val2)) => val1 == val2,
            (Prefix(val1), Prefix(val2)) => val1 == val2,
            (SExpression(val1), SExpression(val2)) => val1 == val2,
            _ => false
        }
    }
}

#[derive(Debug)]
pub struct Code {
    elements: Vec<Element>
}

impl Code {
    pub fn new(elements: Vec<Element>) -> Code {
        Code {
            elements
        }
    }

    pub fn get_elements(&self) -> &Vec<Element> {
        &self.elements
    }
}

pub fn parse_element(s: &str) -> Result<(&str, Element), nom::Err<(&str, nom::error::ErrorKind)>> {
    let int_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        integer_element::parse_integer_element,
        |el| Ok(Element::Integer(el))
    );

    let float_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        float_element::parse_float_element,
        |el| Ok(Element::Float(el))
    );

    let boolean_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        boolean_element::parse_boolean_element,
        |el| Ok(Element::Boolean(el))
    );

    let symbol_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        symbol_element::parse_symbol_element,
        |el| Ok(Element::Symbol(el))
    );

    let keyword_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        keyword_element::parse_keyword_element,
        |el| Ok(Element::Keyword(el))
    );

    let string_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        string_element::parse_string_element,
        |el| Ok(Element::String(el))
    );

    let s_expression_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        s_expression_element::parse_s_expression_element,
        |el| Ok(Element::SExpression(el))
    );

    let quoted_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        prefix_element::parse_prefixed_element,
        |el| Ok(Element::Prefix(el))
    );

    let parser = alt((
        float_parser,
        int_parser,
        boolean_parser,
        symbol_parser,
        keyword_parser,
        string_parser,
        s_expression_parser,
        quoted_parser
    ));

    parser(s)
}

fn make_program(elements: Vec<Element>) -> Result<Code, String> {
    Ok(Code::new(elements))
}

pub fn parse_code(s: &str) -> Result<(&str, Code), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse_element_spaced = preceded(space0, parse_element);
    let parse = many0(parse_element_spaced);

    let parse = map_res(parse, make_program);

    parse(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_is_ok {
        ($exp:expr) => {
            assert!(match $exp {
                Ok(_) => true,
                _ => false
            });
        }
    }

    #[test]
    fn test_simple_s_expressions() {
        assert_is_ok!(parse_element("20"));
        assert_is_ok!(parse_element("20.0"));
        assert_is_ok!(parse_element("#t"));
        assert_is_ok!(parse_element("#f"));
        assert_is_ok!(parse_element("imacutesymbol"));
        assert_is_ok!(parse_element(":imacutekeyword"));
        assert_is_ok!(parse_element(r#""imacutestring""#));
        assert_is_ok!(parse_element("'imaquotedsymbol"));
        assert_is_ok!(parse_element("`imaquotedsymboltoo"));
        assert_is_ok!(parse_element(",imanexecutedsymbol"));
        assert_is_ok!(parse_element(",@imanexecutedsymbolthatexpandstolist"));
    }

    #[test]
    fn test_simple_program() {
//        assert_is_ok!(parse_program("() () ()"));
//        assert_is_ok!(parse_program("() () ("));

        println!("{:?}", parse_code("(plus 1 2)"));
    }

}
