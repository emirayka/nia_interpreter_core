use nom::{
    bytes::complete::tag,
    combinator::{
        map_res
    },
    multi::{
        many1
    },
    sequence::{
        preceded,
        pair
    },
};

use crate::parser::symbol_element::{parse_symbol_element, SymbolElement};

#[derive(Debug)]
pub struct DelimitedSymbolsElement {
    values: Vec<SymbolElement>,
}

impl DelimitedSymbolsElement {
    pub fn new(values: Vec<SymbolElement>) -> DelimitedSymbolsElement {
        DelimitedSymbolsElement {
            values
        }
    }

    pub fn get_symbols(&self) -> &Vec<SymbolElement> {
        &self.values
    }
}

impl PartialEq for DelimitedSymbolsElement {
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

fn make_delimited_symbols_element(pair: (SymbolElement, Vec<SymbolElement>)) -> Result<DelimitedSymbolsElement, String> {
    let mut parts = pair.1;
    parts.insert(0, pair.0);

    Ok(DelimitedSymbolsElement::new(parts))
}

pub fn parse_delimited_symbols_element(
    s: &str
) -> Result<(&str, DelimitedSymbolsElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse_part = preceded(
        tag(":"),
        parse_symbol_element
    );

    let parse_delimited_symbols = pair(
        parse_symbol_element,
        many1(parse_part)
    );

    let parse_delimited_symbols_element = map_res(
        parse_delimited_symbols,
        make_delimited_symbols_element
    );

    parse_delimited_symbols_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_parsing_of_delimited_symbols_element {
        ($symbol_names:expr, $code:expr, $rest:expr) => {
            let expected = Ok(($rest, DelimitedSymbolsElement::new(
                $symbol_names
                    .into_iter()
                    .map(|name| SymbolElement::new(String::from(name)))
                    .collect::<Vec<SymbolElement>>()
            )));

            assert_eq!(expected, parse_delimited_symbols_element($code));
        };
        ($symbol_names:expr, $code:expr) => {
            assert_parsing_of_delimited_symbols_element!($symbol_names, $code, "");
        };
    }

    #[test]
    fn parses_several_delimited_symbols() {
        assert_parsing_of_delimited_symbols_element!(
            vec!(
                "object",
                "value1",
            ),
            "object:value1"
        );
        assert_parsing_of_delimited_symbols_element!(
            vec!(
                "object",
                "value1",
                "value2",
            ),
            "object:value1:value2"
        );
    }

    #[test]
    fn does_not_parse_just_a_symbol() {
        let result = parse_delimited_symbols_element("object");

        assert_eq!(Err(nom::Err::Error(("", nom::error::ErrorKind::Tag))), result);
    }
}
