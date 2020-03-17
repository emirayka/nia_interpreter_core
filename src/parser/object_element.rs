use crate::parser::keyword_element::{KeywordElement, parse_keyword_element};
use crate::parser::{Element, parse_element};
use nom::sequence::{terminated, preceded, pair};
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::combinator::map_res;

#[derive(Debug)]
pub struct ObjectElement {
    values: Vec<(KeywordElement, Element)>,
}

impl ObjectElement {
    pub fn new(values: Vec<(KeywordElement, Element)>) -> ObjectElement {
        ObjectElement {
            values
        }
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

fn make_object_element(values: Vec<(KeywordElement, Element)>) -> Result<ObjectElement, String> {
    Ok(ObjectElement::new(values))
}

pub fn parse_object_element(s: &str) -> Result<(&str, ObjectElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse_pairs = many0(pair(
        preceded(multispace0, parse_keyword_element),
        preceded(multispace0, parse_element)
    ));

    let opening_brace = terminated(tag("{"), multispace0);
    let closing_brace = preceded(multispace0, tag("}"));

    let parse_object = preceded(
        opening_brace,
        terminated(
            parse_pairs,
            closing_brace
        )
    );

    let parse_object_element = map_res(parse_object, make_object_element);

    parse_object_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_parsed_correctly(expr: &str) {
        assert!(parse_object_element(expr).is_ok());
    }

    fn assert_failed_correctly(expr: &str) {
        assert!(parse_object_element(expr).is_err());
    }

    #[test]
    fn simple_object_1() {
        assert_parsed_correctly(r#"{:f :keyword}"#);
    }

    #[test]
    fn simple_object_2() {
        assert_parsed_correctly(r#"{:oi test :a 1 :b 1.0 :c "test" :d #t :e #f :f :keyword}"#);
    }

    #[test]
    fn spaces_are_processed_correctly() {
        let specs = vec!(
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
        );

        for spec in specs {
            assert_parsed_correctly(spec);
        }
    }

    #[test]
    fn nested_objects_are_processed() {
        assert_parsed_correctly(r#"{:a test :b :list :c {:a b :b 1 :c 2}}"#);
        assert_parsed_correctly(r#"{:a test :b {:a test :b {:a b :b 1 :c 2}}}"#);
    }

    #[test]
    fn returns_err_when_pairs_are_incomplete() {
        assert_failed_correctly(r#"{:a test :b}"#);
    }
}
