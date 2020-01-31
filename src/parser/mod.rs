mod lib;

pub mod integer_element;
pub mod float_element;
pub mod symbol_element;
pub mod keyword_element;
pub mod string_element;
pub mod boolean_element;
pub mod s_expression_element;
pub mod prefix_element;

use nom::{
    bytes::complete::tag,
    branch::alt,
    multi::many0,
    combinator::{
        peek,
        map_res,
        all_consuming,
    },
    sequence::{
        terminated,
        preceded,
    },
    character::complete::{
        space0,
        space1,
    },
};

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
        terminated(
            integer_element::parse_integer_element,
            alt((
                peek(space1),
                peek(tag(")")),
                all_consuming(tag(""))
    ))),
        |el| Ok(Element::Integer(el))
    );

    let float_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        terminated(
            float_element::parse_float_element,
            alt((
                peek(space1),
                peek(tag(")")),
                all_consuming(tag(""))
            ))),
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
        boolean_parser,
        float_parser,
        int_parser,
        string_parser,
        keyword_parser,
        s_expression_parser,
        quoted_parser,
        symbol_parser,
    ));

    parser(s)
}

fn make_program(elements: Vec<Element>) -> Result<Code, String> {
    Ok(Code::new(elements))
}

pub fn parse_code(s: &str) -> Result<(&str, Code), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse = many0(preceded(space0, parse_element));

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
        assert_is_ok!(parse_code("20"));
        assert_is_ok!(parse_code("20.0"));
        assert_is_ok!(parse_code("#t"));
        assert_is_ok!(parse_code("#f"));
        assert_is_ok!(parse_code("imacutesymbol"));
        assert_is_ok!(parse_code(":imacutekeyword"));
        assert_is_ok!(parse_code(r#""imacutestring""#));
        assert_is_ok!(parse_code("'imaquotedsymbol"));
        assert_is_ok!(parse_code("`imaquotedsymboltoo"));
        assert_is_ok!(parse_code(",imanexecutedsymbol"));
        assert_is_ok!(parse_code(",@imanexecutedsymbolthatexpandstolist"));
    }

    #[test]
    fn test_simple_program() {
//        assert_is_ok!(parse_code("(+ 1 2)"));
//        assert_is_ok!(parse_code("(1+ 1)"));
    }

    macro_rules! assert_code {
        ($expected:expr, $code:expr) => {
            {
                let expected = $expected;
                let parsed = parse_code($code);

                assert!(parsed.is_ok());
                let parsed = match parsed {
                    Ok((_, code)) => code,
                    _ => unreachable!()
                };

                assert_eq!(expected.len(), parsed.get_elements().len());
                let len = expected.len();

                for i in 0..len {
                    assert_eq!(&expected[i], &parsed.get_elements()[i]);
                }
            };
        }
    }

    #[test]
    fn test_distinguishes_between_symbols_and_numbers() {
        assert_code!(vec!(Element::Float(float_element::FloatElement::new(1.1))), "1.1");
        assert_code!(vec!(Element::Symbol(symbol_element::SymbolElement::new("1.1t".to_string()))), "1.1t");

        assert_code!(vec!(Element::Integer(integer_element::IntegerElement::new(1))), "1");
        assert_code!(vec!(Element::Symbol(symbol_element::SymbolElement::new("1t".to_string()))), "1t");
    }

    #[test]
    fn test_parses_symbol_correctly() {
        assert_eq!(Ok(("", Element::Symbol(symbol_element::SymbolElement::new(String::from("1+"))))), parse_element("1+"));
    }

//    #[test]
//    fn test_escapes_behave_correctlrry() {
//        println!("{:?}", parse_code(r#""a""b""#));
//    }
}
