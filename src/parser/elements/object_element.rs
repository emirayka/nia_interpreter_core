use nom::{
    character::complete::multispace0, character::complete::multispace1,
    delimited, many0, map_res, named, pair, preceded, separated_list,
    separated_pair, tag, terminated,
};

use crate::parser::element;
use crate::parser::element::Element;
use crate::parser::keyword_element;
use crate::parser::keyword_element::KeywordElement;
use crate::parser::ParseError;

#[derive(Debug, Clone)]
pub struct ObjectElement {
    values: Vec<(KeywordElement, Element)>,
}

impl ObjectElement {
    pub fn new(values: Vec<(KeywordElement, Element)>) -> ObjectElement {
        ObjectElement { values }
    }

    pub fn get_values(self) -> Vec<(KeywordElement, Element)> {
        self.values
    }

    pub fn get_values_ref(&self) -> &Vec<(KeywordElement, Element)> {
        &self.values
    }
}

impl PartialEq for ObjectElement {
    fn eq(&self, other: &Self) -> bool {
        if self.values.len() != other.values.len() {
            return false;
        }

        let len = self.values.len();

        for i in 0..len {
            let mut found = false;

            for j in 0..len {
                if self.values[i] == other.values[j] {
                    found = true;
                    break;
                }
            }

            if !found {
                return false;
            }
        }

        return true;
    }
}

impl Eq for ObjectElement {}

fn make_object_element(
    values: Vec<(KeywordElement, Element)>,
) -> Result<ObjectElement, ParseError> {
    Ok(ObjectElement::new(values))
}

named!(parse_pair(&str) -> (KeywordElement, Element), separated_pair!(
    keyword_element::parse,
    multispace1,
    element::parse
));

named!(parse_pairs(&str) -> Vec<(KeywordElement, Element)>, separated_list!(
    multispace1,
    parse_pair
));

named!(parse_opening_brace(&str) -> &str, terminated!(
    tag!("{"),
    multispace0
));

named!(parse_closing_brace(&str) -> &str, preceded!(
    multispace0,
    tag!("}")
));

named!(parse_object(&str) -> Vec<(KeywordElement, Element)>, delimited!(
    parse_opening_brace,
    parse_pairs,
    parse_closing_brace
));

named!(pub parse(&str) -> ObjectElement, map_res!(
    parse_object,
    make_object_element
));

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    fn assert_parsed_correctly(expr: &str) {
        nia_assert(parse(expr).is_ok());
    }

    fn assert_failed_correctly(expr: &str) {
        nia_assert(parse(expr).is_err());
    }

    #[test]
    fn simple_object_1() {
        assert_parsed_correctly(r#"{:f :keyword}"#);
    }

    #[test]
    fn simple_object_2() {
        assert_parsed_correctly(
            r#"{:oi test :a 1 :b 1.0 :c "test" :d #t :e #f :f :keyword}"#,
        );
    }

    #[test]
    fn spaces_are_processed_correctly() {
        let specs = vec![
            "{:a test}",
            "{ :a test}",
            "{:a test }",
            "{ :a test }",
            "{:a\ttest}",
            "{\t:a\ttest}",
            "{:a\ttest\t}",
            "{\t:a\ttest\t}",
            "{:a\rtest}",
            "{\r:a\rtest}",
            "{:a\rtest\r}",
            "{\r:a\rtest\r}",
            "{:a\ntest}",
            "{\n:a\ntest}",
            "{:a\ntest\n}",
            "{\n:a\ntest\n}",
            "{:a\r\ntest}",
            "{\r\n:a\r\ntest}",
            "{:a\r\ntest\r\n}",
            "{\r\n:a\r\ntest\r\n}",
        ];

        for spec in specs {
            assert_parsed_correctly(spec);
        }
    }

    #[test]
    fn nested_objects_are_processed() {
        assert_parsed_correctly(r#"{:a test :b :list :c {:a b :b 1 :c 2}}"#);
        assert_parsed_correctly(
            r#"{:a test :b {:a test :b {:a b :b 1 :c 2}}}"#,
        );
    }

    #[test]
    fn returns_err_when_pairs_are_incomplete() {
        assert_failed_correctly(r#"{:a test :b}"#);
    }
}
