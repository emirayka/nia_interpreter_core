mod lib;

pub mod integer_element;
pub mod float_element;
pub mod symbol_element;
pub mod keyword_element;
pub mod string_element;
pub mod boolean_element;
pub mod s_expression_element;
pub mod object_element;
pub mod prefix_element;
pub mod delimited_symbols_element;

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
    Object(object_element::ObjectElement),
    DelimitedSymbols(delimited_symbols_element::DelimitedSymbolsElement),
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
            (SExpression(val1), SExpression(val2)) => val1 == val2,
            (Object(val1), Object(val2)) => val1 == val2,
            (DelimitedSymbols(val1), DelimitedSymbols(val2)) => val1 == val2,
            (Prefix(val1), Prefix(val2)) => val1 == val2,
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
                peek(tag("}")),
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
                peek(tag("}")),
                all_consuming(tag(""))
            ))),
        |el| Ok(Element::Float(el))
    );

    let boolean_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        terminated(
            boolean_element::parse_boolean_element,
            alt((
                peek(space1),
                peek(tag(")")),
                peek(tag("}")),
                all_consuming(tag(""))
            ))),
        |el| Ok(Element::Boolean(el))
    );

    let symbol_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        terminated(
            symbol_element::parse_symbol_element,
            alt((
                peek(space1),
                peek(tag(")")),
                peek(tag("}")),
                all_consuming(tag(""))
            ))),
        |el| Ok(Element::Symbol(el))
    );

    let keyword_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        terminated(
            keyword_element::parse_keyword_element,
            alt((
                peek(space1),
                peek(tag(")")),
                peek(tag("}")),
                all_consuming(tag(""))
            ))),
        |el| Ok(Element::Keyword(el))
    );

    let string_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        terminated(
            string_element::parse_string_element,
            alt((
                peek(space1),
                peek(tag(")")),
                peek(tag("}")),
                all_consuming(tag(""))
            ))),
        |el| Ok(Element::String(el))
    );

    let s_expression_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        terminated(
            s_expression_element::parse_s_expression_element,
            alt((
                peek(space1),
                peek(tag(")")),
                peek(tag("(")),
                peek(tag("}")),
                all_consuming(tag(""))
            ))),
        |el| Ok(Element::SExpression(el))
    );

    let object_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        terminated(
            object_element::parse_object_element,
            alt((
                peek(space1),
                peek(tag(")")),
                peek(tag("}")),
                all_consuming(tag(""))
            ))),
        |el| Ok(Element::Object(el))
    );

    let delimited_parser = map_res::<_, _, _, _, (&str, nom::error::ErrorKind), _, _>(
        terminated(
            delimited_symbols_element::parse_delimited_symbols_element,
            alt((
                peek(space1),
                peek(tag(")")),
                peek(tag("}")),
                all_consuming(tag(""))
            ))),
        |el| Ok(Element::DelimitedSymbols(el))
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
        object_parser,
        quoted_parser,
        symbol_parser,
        delimited_parser,
    ));

    parser(s)
}

fn make_program(elements: Vec<Element>) -> Result<Code, String> {
    Ok(Code::new(elements))
}

pub fn parse_code(s: &str) -> Result<(&str, Code), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse = many0(preceded(space0, parse_element));

    let parse = map_res(parse, make_program);

    let result = parse(s);

    match result {
        Ok((rest, parse_result)) => {
            if rest.len() != 0 {
                return Err(nom::Err::Failure((rest, nom::error::ErrorKind::NonEmpty)))
            }

            Ok((rest, parse_result))
        },
        x => x
    }
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

    macro_rules! assert_is_err {
        ($exp:expr) => {
            assert!(match $exp {
                Err(_) => true,
                _ => false
            });
        }
    }

    macro_rules! assert_code_eq {
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
    fn test_parses_atoms_correctly() {
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
    fn test_parses_simple_s_expression_correctly() {
        assert_is_ok!(parse_code("(+ 1 2)"));
        assert_is_ok!(parse_code("(1+ 1)"));
    }

    #[test]
    fn test_parses_complex_s_expression_correctly() {
        assert_code_eq!(
            vec!(
                Element::SExpression(s_expression_element::SExpressionElement::new(vec!(
                    Element::Integer(integer_element::IntegerElement::new(1)),
                    Element::Float(float_element::FloatElement::new(1.1)),
                    Element::Boolean(boolean_element::BooleanElement::new(true)),
                    Element::Boolean(boolean_element::BooleanElement::new(false)),
                    Element::Keyword(keyword_element::KeywordElement::new(String::from("keyword"))),
                    Element::String(string_element::StringElement::new(String::from("string"))),
                    Element::Symbol(symbol_element::SymbolElement::new(String::from("symbol"))),
                    Element::SExpression(s_expression_element::SExpressionElement::new(vec!(
                        Element::Integer(integer_element::IntegerElement::new(1)),
                        Element::Float(float_element::FloatElement::new(1.1)),
                        Element::Boolean(boolean_element::BooleanElement::new(true)),
                        Element::Boolean(boolean_element::BooleanElement::new(false)),
                        Element::Keyword(keyword_element::KeywordElement::new(String::from("nested-keyword"))),
                        Element::String(string_element::StringElement::new(String::from("nested-string"))),
                        Element::Symbol(symbol_element::SymbolElement::new(String::from("nested-symbol"))),
                    ))),
                    Element::Object(object_element::ObjectElement::new(vec!(
                        (keyword_element::KeywordElement::new(String::from("a")), Element::Integer(integer_element::IntegerElement::new(1))),
                        (keyword_element::KeywordElement::new(String::from("b")), Element::Float(float_element::FloatElement::new(1.1))),
                        (keyword_element::KeywordElement::new(String::from("c")), Element::Boolean(boolean_element::BooleanElement::new(true))),
                        (keyword_element::KeywordElement::new(String::from("d")), Element::Boolean(boolean_element::BooleanElement::new(false))),
                        (keyword_element::KeywordElement::new(String::from("e")), Element::Keyword(keyword_element::KeywordElement::new(String::from("object-keyword")))),
                        (keyword_element::KeywordElement::new(String::from("f")), Element::String(string_element::StringElement::new(String::from("object-string")))),
                        (keyword_element::KeywordElement::new(String::from("g")), Element::Symbol(symbol_element::SymbolElement::new(String::from("object-symbol")))),
                    )))
                )))
            ),
            "(1 1.1 #t #f :keyword \"string\" symbol (1 1.1 #t #f :nested-keyword \"nested-string\" nested-symbol) {:a 1 :b 1.1 :c #t :d #f :e :object-keyword :f \"object-string\" :g object-symbol})"
        );
    }

    #[test]
    fn test_distinguishes_between_symbols_and_numbers() {
        assert_code_eq!(vec!(Element::Float(float_element::FloatElement::new(1.1))), "1.1");
        assert_code_eq!(vec!(Element::Symbol(symbol_element::SymbolElement::new("1.1t".to_string()))), "1.1t");

        assert_code_eq!(vec!(Element::Integer(integer_element::IntegerElement::new(1))), "1");
        assert_code_eq!(vec!(Element::Symbol(symbol_element::SymbolElement::new("1t".to_string()))), "1t");
    }

    #[test]
    fn test_respects_spaces_between_forms_after_integer() {
        assert_is_ok!(parse_code("1 1"));
        assert_is_ok!(parse_code("11"));

        assert_is_ok!(parse_code("1 1.1"));
        assert_is_ok!(parse_code("11.1"));

        assert_is_ok!(parse_code("1 #t"));
        assert_is_err!(parse_code("1#t"));
        assert_is_ok!(parse_code("1 #f"));
        assert_is_err!(parse_code("1#f"));

        assert_is_ok!(parse_code("1 :t"));
        assert_is_ok!(parse_code("1:t")); // because, "1:t" is neither a valid symbol nor keyword, but it's a valid delimited symbol expression
        // todo: maybe change that

        assert_is_ok!(parse_code("1 t"));
        assert_is_ok!(parse_code("1t")); // because, "1t" is a valid symbol

        assert_is_ok!(parse_code("1 a:b"));
        assert_is_ok!(parse_code("1a:b"));

        assert_is_ok!(parse_code("1 \"\""));
        assert_is_err!(parse_code("1\"\""));

        assert_is_ok!(parse_code("1 ()"));
        assert_is_err!(parse_code("1()"));

        assert_is_ok!(parse_code("1 {}"));
        assert_is_err!(parse_code("1{}"));
    }

    #[test]
    fn test_respects_spaces_between_forms_after_float() {
        assert_is_ok!(parse_code("1.1 1"));
        assert_is_ok!(parse_code("1.11"));

        assert_is_ok!(parse_code("1.1 1.1"));
        assert_is_ok!(parse_code("1.11.1"));

        assert_is_ok!(parse_code("1.1 #t"));
        assert_is_err!(parse_code("1.1#t"));
        assert_is_ok!(parse_code("1.1 #f"));
        assert_is_err!(parse_code("1.1#f"));

        assert_is_ok!(parse_code("1.1 :t"));
        assert_is_ok!(parse_code("1.1:t")); // because, "1:t" is neither a valid symbol nor keyword, but it's a valid delimited symbol expression
        // todo: maybe change that

        assert_is_ok!(parse_code("1.1 t"));
        assert_is_ok!(parse_code("1.1t")); // because, "1t" is a valid symbol

        assert_is_ok!(parse_code("1.1 a:b"));
        assert_is_ok!(parse_code("1.1a:b"));

        assert_is_ok!(parse_code("1.1 \"\""));
        assert_is_err!(parse_code("1.1\"\""));

        assert_is_ok!(parse_code("1.1 ()"));
        assert_is_err!(parse_code("1.1()"));

        assert_is_ok!(parse_code("1.1 {}"));
        assert_is_err!(parse_code("1.1{}"));
    }

    #[test]
    fn test_respects_spaces_between_forms_after_boolean_true() {
        assert_is_ok!(parse_code("#t 1"));
        assert_is_err!(parse_code("#t1"));

        assert_is_ok!(parse_code("#t 1.1"));
        assert_is_err!(parse_code("#t1.1"));

        assert_is_ok!(parse_code("#t #t"));
        assert_is_err!(parse_code("#t#t"));
        assert_is_ok!(parse_code("#t #f"));
        assert_is_err!(parse_code("#t#f"));

        assert_is_ok!(parse_code("#t :t"));
        assert_is_err!(parse_code("#t:t"));

        assert_is_ok!(parse_code("#t t"));
        assert_is_err!(parse_code("#tt"));

        assert_is_ok!(parse_code("#t a:b"));
        assert_is_err!(parse_code("#ta:b"));

        assert_is_ok!(parse_code("#t \"\""));
        assert_is_err!(parse_code("#t\"\""));

        assert_is_ok!(parse_code("#t ()"));
        assert_is_err!(parse_code("#t()"));

        assert_is_ok!(parse_code("#t {}"));
        assert_is_err!(parse_code("#t{}"));
    }

    #[test]
    fn test_respects_spaces_between_forms_after_boolean_false() {
        assert_is_ok!(parse_code("#f 1"));
        assert_is_err!(parse_code("#f1"));

        assert_is_ok!(parse_code("#f 1.1"));
        assert_is_err!(parse_code("#f1.1"));

        assert_is_ok!(parse_code("#f #t"));
        assert_is_err!(parse_code("#f#t"));
        assert_is_ok!(parse_code("#f #f"));
        assert_is_err!(parse_code("#f#f"));

        assert_is_ok!(parse_code("#f :t"));
        assert_is_err!(parse_code("#f:t"));

        assert_is_ok!(parse_code("#f t"));
        assert_is_err!(parse_code("#ft"));

        assert_is_ok!(parse_code("#f a:b"));
        assert_is_err!(parse_code("#fa:b"));

        assert_is_ok!(parse_code("#f \"\""));
        assert_is_err!(parse_code("#f\"\""));

        assert_is_ok!(parse_code("#f ()"));
        assert_is_err!(parse_code("#f()"));

        assert_is_ok!(parse_code("#f {}"));
        assert_is_err!(parse_code("#f{}"));
    }

    #[test]
    fn test_respects_spaces_between_forms_after_keyword() {
        assert_is_ok!(parse_code(":key 1"));
        assert_is_ok!(parse_code(":key1"));

        assert_is_ok!(parse_code(":key 1.1"));
        assert_is_ok!(parse_code(":key1.1"));

        assert_is_ok!(parse_code(":key #t"));
        assert_is_err!(parse_code(":key#t"));
        assert_is_ok!(parse_code(":key #f"));
        assert_is_err!(parse_code(":key#f"));

        assert_is_ok!(parse_code(":key :t"));
        assert_is_ok!(parse_code(":key:t"));

        assert_is_ok!(parse_code(":key t"));
        assert_is_ok!(parse_code(":keyt"));

        assert_is_ok!(parse_code(":key a:b"));
        assert_is_ok!(parse_code(":keya:b"));

        assert_is_ok!(parse_code(":key \"\""));
        assert_is_err!(parse_code(":key\"\""));

        assert_is_ok!(parse_code(":key ()"));
        assert_is_err!(parse_code(":key()"));

        assert_is_ok!(parse_code(":key {}"));
        assert_is_err!(parse_code(":key{}"));
    }

    #[test]
    fn test_respects_spaces_between_forms_after_string() {
        assert_is_ok!(parse_code("\"str\" 1"));
        assert_is_err!(parse_code("\"str\"1"));

        assert_is_ok!(parse_code("\"str\" 1.1"));
        assert_is_err!(parse_code("\"str\"1.1"));

        assert_is_ok!(parse_code("\"str\" #t"));
        assert_is_err!(parse_code("\"str\"#t"));
        assert_is_ok!(parse_code("\"str\" #f"));
        assert_is_err!(parse_code("\"str\"#f"));

        assert_is_ok!(parse_code("\"str\" :t"));
        assert_is_err!(parse_code("\"str\":t"));

        assert_is_ok!(parse_code("\"str\" t"));
        assert_is_err!(parse_code("\"str\"t"));

        assert_is_ok!(parse_code("\"str\" a:b"));
        assert_is_err!(parse_code("\"str\"a:b"));

        assert_is_ok!(parse_code("\"str\" \"\""));
        assert_is_err!(parse_code("\"str\"\"\""));

        assert_is_ok!(parse_code("\"str\" ()"));
        assert_is_err!(parse_code("\"str\"()"));

        assert_is_ok!(parse_code("\"str\" {}"));
        assert_is_err!(parse_code("\"str\"{}"));
    }

    #[test]
    fn test_respects_spaces_between_forms_after_symbol() {
        assert_is_ok!(parse_code("sym 1"));
        assert_is_ok!(parse_code("sym1"));

        assert_is_ok!(parse_code("sym 1.1"));
        assert_is_ok!(parse_code("sym1.1"));

        assert_is_ok!(parse_code("sym #t"));
        assert_is_err!(parse_code("sym#t"));
        assert_is_ok!(parse_code("sym #f"));
        assert_is_err!(parse_code("sym#f"));

        assert_is_ok!(parse_code("sym :t"));
        assert_is_ok!(parse_code("sym:t")); // it's a valid delimited symbol expression

        assert_is_ok!(parse_code("sym t"));
        assert_is_ok!(parse_code("symt"));

        assert_is_ok!(parse_code("sym a:b"));
        assert_is_ok!(parse_code("syma:b"));

        assert_is_ok!(parse_code("sym \"\""));
        assert_is_err!(parse_code("sym\"\""));

        assert_is_ok!(parse_code("sym ()"));
        assert_is_err!(parse_code("sym()"));

        assert_is_ok!(parse_code("sym {}"));
        assert_is_err!(parse_code("sym{}"));
    }

    #[test]
    fn test_respects_spaces_between_forms_after_s_expresion() {
        assert_is_ok!(parse_code("() 1"));
        assert_is_err!(parse_code("()1"));

        assert_is_ok!(parse_code("() 1.1"));
        assert_is_err!(parse_code("()1.1"));

        assert_is_ok!(parse_code("() #t"));
        assert_is_err!(parse_code("()#t"));
        assert_is_ok!(parse_code("() #f"));
        assert_is_err!(parse_code("()#f"));

        assert_is_ok!(parse_code("() :t"));
        assert_is_err!(parse_code("():t"));

        assert_is_ok!(parse_code("() t"));
        assert_is_err!(parse_code("()t"));

        assert_is_ok!(parse_code("() a:b"));
        assert_is_err!(parse_code("()a:b"));

        assert_is_ok!(parse_code("() \"\""));
        assert_is_err!(parse_code("()\"\""));

        assert_is_ok!(parse_code("() ()"));
        assert_is_ok!(parse_code("()()"));

        assert_is_ok!(parse_code("() {}"));
        assert_is_err!(parse_code("(){}"));
    }

    #[test]
    fn test_respects_spaces_between_forms_after_object_literal() {
        assert_is_ok!(parse_code("{} 1"));
        assert_is_err!(parse_code("{}1"));

        assert_is_ok!(parse_code("{} 1.1"));
        assert_is_err!(parse_code("{}1.1"));

        assert_is_ok!(parse_code("{} #t"));
        assert_is_err!(parse_code("{}#t"));
        assert_is_ok!(parse_code("{} #f"));
        assert_is_err!(parse_code("{}#f"));

        assert_is_ok!(parse_code("{} :t"));
        assert_is_err!(parse_code("{}:t"));

        assert_is_ok!(parse_code("{} t"));
        assert_is_err!(parse_code("{}t"));

        assert_is_ok!(parse_code("{} a:b"));
        assert_is_err!(parse_code("{}a:b"));

        assert_is_ok!(parse_code("{} \"\""));
        assert_is_err!(parse_code("{}\"\""));

        assert_is_ok!(parse_code("{} ()"));
        assert_is_err!(parse_code("{}()"));

        assert_is_ok!(parse_code("{} {}"));
        assert_is_err!(parse_code("{}{}"));
    }

    #[test]
    fn test_does_not_allow_unfinished_s_expressions() {
        assert_is_err!(parse_code("("));
        assert_is_err!(parse_code("()("));
        assert_is_err!(parse_code("(()"));
        assert_is_err!(parse_code("\"string")); // todo: move to another test
        assert_is_err!(parse_code("((\"string))"));
    }

    #[test]
    fn test_does_not_allow_unfinished_object_literals() {
        assert_is_err!(parse_code("{"));
        assert_is_err!(parse_code("{}{"));
        assert_is_err!(parse_code("{{}"));
        assert_is_err!(parse_code("\"string"));
        assert_is_err!(parse_code("{{\"string}}"));
    }

    // todo: add tests when input is not complete
}
