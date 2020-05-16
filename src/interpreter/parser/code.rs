use nom::alt;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::complete;
use nom::delimited;
use nom::many0;
use nom::map_res;
use nom::named;
use nom::preceded;
use nom::tag;

use crate::interpreter::parser::element;
use crate::interpreter::parser::element::Element;
use crate::interpreter::parser::ParseError;

use crate::interpreter::parser::lib::parse_comment_character;

#[derive(Debug)]
pub struct Code {
    elements: Vec<Element>,
}

impl Code {
    pub fn new(elements: Vec<Element>) -> Code {
        Code { elements }
    }

    pub fn get_elements(self) -> Vec<Element> {
        self.elements
    }
}

fn make_none(_: Vec<char>) -> Result<Option<Element>, ParseError> {
    Ok(None)
}

fn make_some(element: Element) -> Result<Option<Element>, ParseError> {
    Ok(Some(element))
}

fn make_code(
    probably_elements: Vec<Option<Element>>,
) -> Result<Code, ParseError> {
    let mut elements = Vec::new();

    for element in probably_elements {
        match element {
            Some(element) => {
                elements.push(element);
            }
            None => {}
        }
    }
    Ok(Code::new(elements))
}

named!(parse_comment_chars(&str) -> Vec<char>, many0!(
    parse_comment_character
));

named!(parse_comment(&str) -> Vec<char>, preceded!(
    tag!(";"),
    parse_comment_chars
));

named!(parse_comment_element(&str) -> Option<Element>, map_res!(
    parse_comment,
    make_none
));

named!(parse_noncomment_element(&str) -> Option<Element>, map_res!(
    element::parse,
    make_some
));

named!(parse_element(&str) -> Option<Element>, alt!(
    parse_comment_element |
    parse_noncomment_element
));

named!(parse_elements(&str) -> Vec<Option<Element>>, many0!(
    preceded!(
        multispace0,
        complete!(parse_element)
    )
));

named!(parse_code(&str) -> Code, map_res!(
    delimited!(
        multispace0,
        parse_elements,
        multispace0
    ),
    make_code
));

pub fn parse(s: &str) -> Result<Code, ParseError> {
    let result = parse_code(s);

    match result {
        Ok((rest, parse_result)) => {
            if rest.len() != 0 {
                return Err(ParseError::TrailingInput(String::from(rest)));
            }

            Ok(parse_result)
        }
        Err(nom::Err::Error((s, kind))) => {
            Err(ParseError::NomError((String::from(s), kind)))
        }
        Err(nom::Err::Failure((s, kind))) => {
            Err(ParseError::NomFailure((String::from(s), kind)))
        }
        Err(nom::Err::Incomplete(_)) => Err(ParseError::NomIncomplete()),
    }
}
