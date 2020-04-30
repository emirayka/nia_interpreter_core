use nom::{
    named,
    map_res,
    preceded,
    delimited,
    complete,
    many0,
    character::complete::multispace1,
    character::complete::multispace0,
};

use crate::parser::element;
use crate::parser::element::Element;
use crate::parser::ParseError;

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

    pub fn get_elements(self) -> Vec<Element> {
        self.elements
    }
}

fn make_code(elements: Vec<Element>) -> Result<Code, ParseError>{
    Ok(Code::new(elements))
}

named!(parse_elements(&str) -> Vec<Element>, many0!(
    preceded!(
        multispace0,
        complete!(element::parse)
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

pub fn parse(s: &str) -> Result<(&str, Code), ParseError> {
    let result = parse_code(s);

    match result {
        Ok((rest, parse_result)) => {
            if rest.len() != 0 {
                return Err(ParseError::TrailingInput(String::from(rest)))
            }

            Ok((rest, parse_result))
        },
        Err(nom::Err::Error((s, kind))) => Err(ParseError::NomError((String::from(s), kind))),
        Err(nom::Err::Failure((s, kind))) => Err(ParseError::NomFailure((String::from(s), kind))),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::NomIncomplete())
    }
}
