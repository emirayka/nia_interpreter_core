use nom::{
    named,
    map_res,
    tag,
    many1,
    preceded,
    pair,
    complete,
};

use crate::parser::{symbol_element, ParseError};
use crate::parser::symbol_element::SymbolElement;

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

impl Eq for DelimitedSymbolsElement {}

fn make_delimited_symbols_element(
    pairs: (SymbolElement, Vec<SymbolElement>)
) -> Result<DelimitedSymbolsElement, ParseError> {
    let mut symbols = pairs.1;

    symbols.insert(0, pairs.0);

    Ok(DelimitedSymbolsElement::new(symbols))
}

named!(tt(&str) -> SymbolElement,
    complete!(
        preceded!(
            tag!(":"),
            symbol_element::parse
        )
    )
);

named!(parse_rest_symbols(&str) -> Vec<SymbolElement>, many1!(tt));

named!(pub parse(&str) -> DelimitedSymbolsElement, map_res!(
    pair!(
        symbol_element::parse,
        parse_rest_symbols
    ),
    make_delimited_symbols_element
));

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    #[test]
    fn testw() {
        println!("{:?}", parse("a:barst:c"));
    }

    macro_rules! assert_parsing_of_delimited_symbols_element {
        ($symbol_names:expr, $code:expr, $rest:expr) => {
            let expected = Ok(($rest, DelimitedSymbolsElement::new(
                $symbol_names
                    .into_iter()
                    .map(|name| SymbolElement::new(String::from(name)))
                    .collect::<Vec<SymbolElement>>()
            )));

            nia_assert_equal(expected, parse($code));
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
        let result = parse("object");

        nia_assert(result.is_err());
    }
}
