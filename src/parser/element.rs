use nom::{
    named,
    map_res,
    alt,
    terminated,
    complete,
    peek,
    tag,
    character::complete::multispace1,
};

use super::{
    boolean_element,
    short_lambda_element,
    object_pattern_element,
    float_element,
    integer_element,
    string_element,
    keyword_element,
    s_expression_element,
    object_element,
    prefixed_element,
    delimited_symbols_element,
    symbol_element,

    boolean_element::BooleanElement,
    short_lambda_element::ShortLambdaElement,
    object_pattern_element::ObjectPatternElement,
    float_element::FloatElement,
    integer_element::IntegerElement,
    string_element::StringElement,
    keyword_element::KeywordElement,
    s_expression_element::SExpressionElement,
    object_element::ObjectElement,
    prefixed_element::PrefixedElement,
    delimited_symbols_element::DelimitedSymbolsElement,
    symbol_element::SymbolElement,
};

use crate::parser::ParseError;
use crate::parser::lib::end_of_input;

#[derive(Debug)]
pub enum Element {
    Boolean(BooleanElement),
    ShortLambda(ShortLambdaElement),
    ObjectPattern(ObjectPatternElement),
    Float(FloatElement),
    Integer(IntegerElement),
    String(StringElement),
    Keyword(KeywordElement),
    SExpression(SExpressionElement),
    Object(ObjectElement),
    Prefix(PrefixedElement),
    DelimitedSymbols(DelimitedSymbolsElement),
    Symbol(SymbolElement),
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        use Element::*;

        match (self, other) {
            (Boolean(val1), Boolean(val2)) => val1 == val2,
            (ShortLambda(val1), ShortLambda(val2)) => val1 == val2,
            (ObjectPattern(val1), ObjectPattern(val2)) => val1 == val2,
            (Float(val1), Float(val2)) => val1 == val2,
            (Integer(val1), Integer(val2)) => val1 == val2,
            (String(val1), String(val2)) => val1 == val2,
            (Keyword(val1), Keyword(val2)) => val1 == val2,
            (SExpression(val1), SExpression(val2)) => val1 == val2,
            (Object(val1), Object(val2)) => val1 == val2,
            (Prefix(val1), Prefix(val2)) => val1 == val2,
            (DelimitedSymbols(val1), DelimitedSymbols(val2)) => val1 == val2,
            (Symbol(val1), Symbol(val2)) => val1 == val2,
            _ => false
        }
    }
}

fn make_boolean_element(el: BooleanElement) -> Result<Element, ParseError> {
    Ok(Element::Boolean(el))
}

fn make_short_lambda_element(el: ShortLambdaElement) -> Result<Element, ParseError> {
    Ok(Element::ShortLambda(el))
}

fn make_object_pattern_element(el: ObjectPatternElement) -> Result<Element, ParseError> {
    Ok(Element::ObjectPattern(el))
}

fn make_float_element(el: FloatElement) -> Result<Element, ParseError> {
    Ok(Element::Float(el))
}

fn make_integer_element(el: IntegerElement) -> Result<Element, ParseError> {
    Ok(Element::Integer(el))
}

fn make_string_element(el: StringElement) -> Result<Element, ParseError> {
    Ok(Element::String(el))
}

fn make_keyword_element(el: KeywordElement) -> Result<Element, ParseError> {
    Ok(Element::Keyword(el))
}

fn make_s_expression_element(el: SExpressionElement) -> Result<Element, ParseError> {
    Ok(Element::SExpression(el))
}

fn make_object_element(el: ObjectElement) -> Result<Element, ParseError> {
    Ok(Element::Object(el))
}

fn make_prefix_element(el: PrefixedElement) -> Result<Element, ParseError> {
    Ok(Element::Prefix(el))
}

fn make_delimited_symbols_element(el: DelimitedSymbolsElement) -> Result<Element, ParseError> {
    Ok(Element::DelimitedSymbols(el))
}

fn make_symbol_element(el: SymbolElement) -> Result<Element, ParseError> {
    Ok(Element::Symbol(el))
}

named!(parse_boolean_element(&str) -> Element, map_res!(
    terminated!(
        boolean_element::parse,
        alt!(
            complete!(peek!(multispace1)) |
            complete!(peek!(tag!(")"))) |
            complete!(peek!(tag!("}"))) |
            end_of_input
        )
    ),
    make_boolean_element
));

named!(parse_short_lambda_element(&str) -> Element, map_res!(
    terminated!(
        short_lambda_element::parse,
        alt!(
            complete!(peek!(multispace1)) |
            complete!(peek!(tag!(")"))) |
            complete!(peek!(tag!("}"))) |
            end_of_input
        )
    ),
    make_short_lambda_element
));

named!(parse_object_pattern_element(&str) -> Element, map_res!(
    terminated!(
        object_pattern_element::parse,
        alt!(
            complete!(peek!(multispace1)) |
            complete!(peek!(tag!(")"))) |
            complete!(peek!(tag!("}"))) |
            end_of_input
        )
    ),
    make_object_pattern_element
));

named!(parse_float_element(&str) -> Element, map_res!(
    terminated!(
        float_element::parse,
        alt!(
            complete!(peek!(multispace1)) |
            complete!(peek!(tag!(")"))) |
            complete!(peek!(tag!("}"))) |
            end_of_input
        )
    ),
    make_float_element
));

named!(parse_integer_element(&str) -> Element, map_res!(
    terminated!(
        integer_element::parse,
        alt!(
            complete!(peek!(multispace1)) |
            complete!(peek!(tag!(")"))) |
            complete!(peek!(tag!("}"))) |
            end_of_input
        )
    ),
    make_integer_element
));

named!(parse_string_element(&str) -> Element, map_res!(
    terminated!(
        string_element::parse,
        alt!(
            complete!(peek!(multispace1)) |
            complete!(peek!(tag!(")"))) |
            complete!(peek!(tag!("}"))) |
            end_of_input
        )
    ),
    make_string_element
));

named!(parse_keyword_element(&str) -> Element, map_res!(
    terminated!(
        keyword_element::parse,
        alt!(
            complete!(peek!(multispace1)) |
            complete!(peek!(tag!(")"))) |
            complete!(peek!(tag!("}"))) |
            end_of_input
        )
    ),
    make_keyword_element
));

named!(parse_s_expression_element(&str) -> Element, map_res!(
    terminated!(
        s_expression_element::parse,
        alt!(
            complete!(peek!(multispace1)) |
            complete!(peek!(tag!("("))) |
            complete!(peek!(tag!(")"))) |
            complete!(peek!(tag!("}"))) |
            end_of_input
        )
    ),
    make_s_expression_element
));

named!(parse_object_element(&str) -> Element, map_res!(
    terminated!(
        object_element::parse,
        alt!(
            complete!(peek!(multispace1)) |
            complete!(peek!(tag!(")"))) |
            complete!(peek!(tag!("}"))) |
            end_of_input
        )
    ),
    make_object_element
));

named!(parse_prefix_element(&str) -> Element, map_res!(
    terminated!(
        prefixed_element::parse,
        alt!(
            complete!(peek!(multispace1)) |
            complete!(peek!(tag!(")"))) |
            complete!(peek!(tag!("}"))) |
            end_of_input
        )
    ),
    make_prefix_element
));

named!(parse_delimited_symbols_element(&str) -> Element, map_res!(
    terminated!(
        delimited_symbols_element::parse,
        alt!(
            complete!(peek!(multispace1)) |
            complete!(peek!(tag!(")"))) |
            complete!(peek!(tag!("}"))) |
            end_of_input
        )
    ),
    make_delimited_symbols_element
));

named!(parse_symbol_element(&str) -> Element, map_res!(
    terminated!(
        symbol_element::parse,
        alt!(
            complete!(peek!(multispace1)) |
            complete!(peek!(tag!(")"))) |
            complete!(peek!(tag!("}"))) |
            end_of_input
        )
    ),
    make_symbol_element
));

named!(pub parse(&str) -> Element, alt!(
        parse_boolean_element |
        parse_short_lambda_element |
        parse_object_pattern_element |
        parse_float_element |
        parse_integer_element |
        parse_string_element |
        parse_keyword_element |
        parse_s_expression_element |
        parse_object_element |
        parse_prefix_element |
        parse_delimited_symbols_element |
        parse_symbol_element
    )
);
